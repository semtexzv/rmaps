use bytes::{BufMut, BytesMut};
use http::Version;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{mem, ptr, slice};

use httprequest::HttpInnerMessage;

/// Internal use only! unsafe
pub(crate) struct SharedMessagePool(RefCell<VecDeque<Rc<HttpInnerMessage>>>);

impl SharedMessagePool {
    pub fn new() -> SharedMessagePool {
        SharedMessagePool(RefCell::new(VecDeque::with_capacity(128)))
    }

    #[inline]
    pub fn get(&self) -> Rc<HttpInnerMessage> {
        if let Some(msg) = self.0.borrow_mut().pop_front() {
            msg
        } else {
            Rc::new(HttpInnerMessage::default())
        }
    }

    #[inline]
    pub fn release(&self, mut msg: Rc<HttpInnerMessage>) {
        let v = &mut self.0.borrow_mut();
        if v.len() < 128 {
            Rc::get_mut(&mut msg).unwrap().reset();
            v.push_front(msg);
        }
    }
}

pub(crate) struct SharedHttpInnerMessage(
    Option<Rc<HttpInnerMessage>>,
    Option<Rc<SharedMessagePool>>,
);

impl Drop for SharedHttpInnerMessage {
    fn drop(&mut self) {
        if let Some(ref pool) = self.1 {
            if let Some(msg) = self.0.take() {
                if Rc::strong_count(&msg) == 1 {
                    pool.release(msg);
                }
            }
        }
    }
}

impl Clone for SharedHttpInnerMessage {
    fn clone(&self) -> SharedHttpInnerMessage {
        SharedHttpInnerMessage(self.0.clone(), self.1.clone())
    }
}

impl Default for SharedHttpInnerMessage {
    fn default() -> SharedHttpInnerMessage {
        SharedHttpInnerMessage(Some(Rc::new(HttpInnerMessage::default())), None)
    }
}

impl SharedHttpInnerMessage {
    pub fn from_message(msg: HttpInnerMessage) -> SharedHttpInnerMessage {
        SharedHttpInnerMessage(Some(Rc::new(msg)), None)
    }

    pub fn new(
        msg: Rc<HttpInnerMessage>, pool: Rc<SharedMessagePool>,
    ) -> SharedHttpInnerMessage {
        SharedHttpInnerMessage(Some(msg), Some(pool))
    }

    #[inline(always)]
    #[allow(mutable_transmutes)]
    #[cfg_attr(feature = "cargo-clippy", allow(mut_from_ref, inline_always))]
    pub fn get_mut(&self) -> &mut HttpInnerMessage {
        let r: &HttpInnerMessage = self.0.as_ref().unwrap().as_ref();
        unsafe { &mut *(r as *const _ as *mut _) }
    }

    #[inline(always)]
    #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
    pub fn get_ref(&self) -> &HttpInnerMessage {
        self.0.as_ref().unwrap()
    }
}

const DEC_DIGITS_LUT: &[u8] = b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

pub(crate) fn write_status_line(version: Version, mut n: u16, bytes: &mut BytesMut) {
    let mut buf: [u8; 13] = [
        b'H', b'T', b'T', b'P', b'/', b'1', b'.', b'1', b' ', b' ', b' ', b' ', b' ',
    ];
    match version {
        Version::HTTP_2 => buf[5] = b'2',
        Version::HTTP_10 => buf[7] = b'0',
        Version::HTTP_09 => {
            buf[5] = b'0';
            buf[7] = b'9';
        }
        _ => (),
    }

    let mut curr: isize = 12;
    let buf_ptr = buf.as_mut_ptr();
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();
    let four = n > 999;

    unsafe {
        // decode 2 more chars, if > 2 chars
        let d1 = (n % 100) << 1;
        n /= 100;
        curr -= 2;
        ptr::copy_nonoverlapping(lut_ptr.offset(d1 as isize), buf_ptr.offset(curr), 2);

        // decode last 1 or 2 chars
        if n < 10 {
            curr -= 1;
            *buf_ptr.offset(curr) = (n as u8) + b'0';
        } else {
            let d1 = n << 1;
            curr -= 2;
            ptr::copy_nonoverlapping(
                lut_ptr.offset(d1 as isize),
                buf_ptr.offset(curr),
                2,
            );
        }
    }

    bytes.put_slice(&buf);
    if four {
        bytes.put(b' ');
    }
}

/// NOTE: bytes object has to contain enough space
pub fn write_content_length(mut n: usize, bytes: &mut BytesMut) {
    if n < 10 {
        let mut buf: [u8; 21] = [
            b'\r', b'\n', b'c', b'o', b'n', b't', b'e', b'n', b't', b'-', b'l', b'e',
            b'n', b'g', b't', b'h', b':', b' ', b'0', b'\r', b'\n',
        ];
        buf[18] = (n as u8) + b'0';
        bytes.put_slice(&buf);
    } else if n < 100 {
        let mut buf: [u8; 22] = [
            b'\r', b'\n', b'c', b'o', b'n', b't', b'e', b'n', b't', b'-', b'l', b'e',
            b'n', b'g', b't', b'h', b':', b' ', b'0', b'0', b'\r', b'\n',
        ];
        let d1 = n << 1;
        unsafe {
            ptr::copy_nonoverlapping(
                DEC_DIGITS_LUT.as_ptr().offset(d1 as isize),
                buf.as_mut_ptr().offset(18),
                2,
            );
        }
        bytes.put_slice(&buf);
    } else if n < 1000 {
        let mut buf: [u8; 23] = [
            b'\r', b'\n', b'c', b'o', b'n', b't', b'e', b'n', b't', b'-', b'l', b'e',
            b'n', b'g', b't', b'h', b':', b' ', b'0', b'0', b'0', b'\r', b'\n',
        ];
        // decode 2 more chars, if > 2 chars
        let d1 = (n % 100) << 1;
        n /= 100;
        unsafe {
            ptr::copy_nonoverlapping(
                DEC_DIGITS_LUT.as_ptr().offset(d1 as isize),
                buf.as_mut_ptr().offset(19),
                2,
            )
        };

        // decode last 1
        buf[18] = (n as u8) + b'0';

        bytes.put_slice(&buf);
    } else {
        bytes.put_slice(b"\r\ncontent-length: ");
        convert_usize(n, bytes);
    }
}

pub(crate) fn convert_usize(mut n: usize, bytes: &mut BytesMut) {
    let mut curr: isize = 39;
    let mut buf: [u8; 41] = unsafe { mem::uninitialized() };
    buf[39] = b'\r';
    buf[40] = b'\n';
    let buf_ptr = buf.as_mut_ptr();
    let lut_ptr = DEC_DIGITS_LUT.as_ptr();

    unsafe {
        // eagerly decode 4 characters at a time
        while n >= 10_000 {
            let rem = (n % 10_000) as isize;
            n /= 10_000;

            let d1 = (rem / 100) << 1;
            let d2 = (rem % 100) << 1;
            curr -= 4;
            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
        }

        // if we reach here numbers are <= 9999, so at most 4 chars long
        let mut n = n as isize; // possibly reduce 64bit math

        // decode 2 more chars, if > 2 chars
        if n >= 100 {
            let d1 = (n % 100) << 1;
            n /= 100;
            curr -= 2;
            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
        }

        // decode last 1 or 2 chars
        if n < 10 {
            curr -= 1;
            *buf_ptr.offset(curr) = (n as u8) + b'0';
        } else {
            let d1 = n << 1;
            curr -= 2;
            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
        }
    }

    unsafe {
        bytes.extend_from_slice(slice::from_raw_parts(
            buf_ptr.offset(curr),
            41 - curr as usize,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_content_length() {
        let mut bytes = BytesMut::new();
        bytes.reserve(50);
        write_content_length(0, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 0\r\n"[..]);
        bytes.reserve(50);
        write_content_length(9, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 9\r\n"[..]);
        bytes.reserve(50);
        write_content_length(10, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 10\r\n"[..]);
        bytes.reserve(50);
        write_content_length(99, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 99\r\n"[..]);
        bytes.reserve(50);
        write_content_length(100, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 100\r\n"[..]);
        bytes.reserve(50);
        write_content_length(101, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 101\r\n"[..]);
        bytes.reserve(50);
        write_content_length(998, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 998\r\n"[..]);
        bytes.reserve(50);
        write_content_length(1000, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 1000\r\n"[..]);
        bytes.reserve(50);
        write_content_length(1001, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 1001\r\n"[..]);
        bytes.reserve(50);
        write_content_length(5909, &mut bytes);
        assert_eq!(bytes.take().freeze(), b"\r\ncontent-length: 5909\r\n"[..]);
    }
}
