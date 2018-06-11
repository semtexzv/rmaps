#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names))]

use bytes::{BufMut, BytesMut};
use futures::{Async, Poll};
use std::io;
use std::rc::Rc;
use tokio_io::AsyncWrite;

use super::encoding::ContentEncoder;
use super::helpers;
use super::settings::WorkerSettings;
use super::shared::SharedBytes;
use super::{Writer, WriterState, MAX_WRITE_BUFFER_SIZE};
use body::{Binary, Body};
use header::ContentEncoding;
use http::header::{HeaderValue, CONNECTION, CONTENT_LENGTH, DATE};
use http::{Method, Version};
use httprequest::HttpInnerMessage;
use httpresponse::HttpResponse;

const AVERAGE_HEADER_SIZE: usize = 30; // totally scientific

bitflags! {
    struct Flags: u8 {
        const STARTED = 0b0000_0001;
        const UPGRADE = 0b0000_0010;
        const KEEPALIVE = 0b0000_0100;
        const DISCONNECTED = 0b0000_1000;
    }
}

pub(crate) struct H1Writer<T: AsyncWrite, H: 'static> {
    flags: Flags,
    stream: T,
    encoder: ContentEncoder,
    written: u64,
    headers_size: u32,
    buffer: SharedBytes,
    buffer_capacity: usize,
    settings: Rc<WorkerSettings<H>>,
}

impl<T: AsyncWrite, H: 'static> H1Writer<T, H> {
    pub fn new(
        stream: T, buf: SharedBytes, settings: Rc<WorkerSettings<H>>,
    ) -> H1Writer<T, H> {
        H1Writer {
            flags: Flags::KEEPALIVE,
            encoder: ContentEncoder::empty(buf.clone()),
            written: 0,
            headers_size: 0,
            buffer: buf,
            buffer_capacity: 0,
            stream,
            settings,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.stream
    }

    pub fn reset(&mut self) {
        self.written = 0;
        self.flags = Flags::KEEPALIVE;
    }

    pub fn disconnected(&mut self) {
        self.buffer.take();
    }

    pub fn keepalive(&self) -> bool {
        self.flags.contains(Flags::KEEPALIVE) && !self.flags.contains(Flags::UPGRADE)
    }

    fn write_data(&mut self, data: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        while written < data.len() {
            match self.stream.write(&data[written..]) {
                Ok(0) => {
                    self.disconnected();
                    return Err(io::Error::new(io::ErrorKind::WriteZero, ""));
                }
                Ok(n) => {
                    written += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return Ok(written)
                }
                Err(err) => return Err(err),
            }
        }
        Ok(written)
    }
}

impl<T: AsyncWrite, H: 'static> Writer for H1Writer<T, H> {
    #[inline]
    fn written(&self) -> u64 {
        self.written
    }

    #[inline]
    fn set_date(&self, dst: &mut BytesMut) {
        self.settings.set_date(dst)
    }

    #[inline]
    fn buffer(&self) -> &mut BytesMut {
        self.buffer.get_mut()
    }

    fn start(
        &mut self, req: &mut HttpInnerMessage, msg: &mut HttpResponse,
        encoding: ContentEncoding,
    ) -> io::Result<WriterState> {
        // prepare task
        self.encoder =
            ContentEncoder::for_server(self.buffer.clone(), req, msg, encoding);
        if msg.keep_alive().unwrap_or_else(|| req.keep_alive()) {
            self.flags = Flags::STARTED | Flags::KEEPALIVE;
        } else {
            self.flags = Flags::STARTED;
        }

        // Connection upgrade
        let version = msg.version().unwrap_or_else(|| req.version);
        if msg.upgrade() {
            self.flags.insert(Flags::UPGRADE);
            msg.headers_mut()
                .insert(CONNECTION, HeaderValue::from_static("upgrade"));
        }
        // keep-alive
        else if self.flags.contains(Flags::KEEPALIVE) {
            if version < Version::HTTP_11 {
                msg.headers_mut()
                    .insert(CONNECTION, HeaderValue::from_static("keep-alive"));
            }
        } else if version >= Version::HTTP_11 {
            msg.headers_mut()
                .insert(CONNECTION, HeaderValue::from_static("close"));
        }
        let body = msg.replace_body(Body::Empty);

        // render message
        {
            let mut buffer = self.buffer.get_mut();
            let reason = msg.reason().as_bytes();
            let mut is_bin = if let Body::Binary(ref bytes) = body {
                buffer.reserve(
                    256
                        + msg.headers().len() * AVERAGE_HEADER_SIZE
                        + bytes.len()
                        + reason.len(),
                );
                true
            } else {
                buffer.reserve(
                    256 + msg.headers().len() * AVERAGE_HEADER_SIZE + reason.len(),
                );
                false
            };

            // status line
            helpers::write_status_line(version, msg.status().as_u16(), &mut buffer);
            SharedBytes::extend_from_slice_(buffer, reason);

            match body {
                Body::Empty => if req.method != Method::HEAD {
                    SharedBytes::put_slice(buffer, b"\r\ncontent-length: 0\r\n");
                } else {
                    SharedBytes::put_slice(buffer, b"\r\n");
                },
                Body::Binary(ref bytes) => {
                    helpers::write_content_length(bytes.len(), &mut buffer)
                }
                _ => SharedBytes::put_slice(buffer, b"\r\n"),
            }

            // write headers
            let mut pos = 0;
            let mut has_date = false;
            let mut remaining = buffer.remaining_mut();
            let mut buf = unsafe { &mut *(buffer.bytes_mut() as *mut [u8]) };
            for (key, value) in msg.headers() {
                if is_bin && key == CONTENT_LENGTH {
                    is_bin = false;
                    continue;
                }
                has_date = has_date || key == DATE;
                let v = value.as_ref();
                let k = key.as_str().as_bytes();
                let len = k.len() + v.len() + 4;
                if len > remaining {
                    unsafe { buffer.advance_mut(pos) };
                    pos = 0;
                    buffer.reserve(len);
                    remaining = buffer.remaining_mut();
                    buf = unsafe { &mut *(buffer.bytes_mut() as *mut _) };
                }

                buf[pos..pos + k.len()].copy_from_slice(k);
                pos += k.len();
                buf[pos..pos + 2].copy_from_slice(b": ");
                pos += 2;
                buf[pos..pos + v.len()].copy_from_slice(v);
                pos += v.len();
                buf[pos..pos + 2].copy_from_slice(b"\r\n");
                pos += 2;
                remaining -= len;
            }
            unsafe { buffer.advance_mut(pos) };

            // optimized date header, set_date writes \r\n
            if !has_date {
                self.settings.set_date(&mut buffer);
            } else {
                // msg eof
                SharedBytes::extend_from_slice_(buffer, b"\r\n");
            }
            self.headers_size = buffer.len() as u32;
        }

        if let Body::Binary(bytes) = body {
            self.written = bytes.len() as u64;
            self.encoder.write(bytes)?;
        } else {
            // capacity, makes sense only for streaming or actor
            self.buffer_capacity = msg.write_buffer_capacity();

            msg.replace_body(body);
        }
        Ok(WriterState::Done)
    }

    fn write(&mut self, payload: Binary) -> io::Result<WriterState> {
        self.written += payload.len() as u64;
        if !self.flags.contains(Flags::DISCONNECTED) {
            if self.flags.contains(Flags::STARTED) {
                // shortcut for upgraded connection
                if self.flags.contains(Flags::UPGRADE) {
                    if self.buffer.is_empty() {
                        let pl: &[u8] = payload.as_ref();
                        let n = self.write_data(pl)?;
                        if n < pl.len() {
                            self.buffer.extend_from_slice(&pl[n..]);
                            return Ok(WriterState::Done);
                        }
                    } else {
                        self.buffer.extend(payload);
                    }
                } else {
                    // TODO: add warning, write after EOF
                    self.encoder.write(payload)?;
                }
            } else {
                // could be response to EXCEPT header
                self.buffer.extend_from_slice(payload.as_ref())
            }
        }

        if self.buffer.len() > self.buffer_capacity {
            Ok(WriterState::Pause)
        } else {
            Ok(WriterState::Done)
        }
    }

    fn write_eof(&mut self) -> io::Result<WriterState> {
        if !self.encoder.write_eof()? {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Last payload item, but eof is not reached",
            ))
        } else if self.buffer.len() > MAX_WRITE_BUFFER_SIZE {
            Ok(WriterState::Pause)
        } else {
            Ok(WriterState::Done)
        }
    }

    #[inline]
    fn poll_completed(&mut self, shutdown: bool) -> Poll<(), io::Error> {
        if !self.buffer.is_empty() {
            let buf: &[u8] =
                unsafe { &mut *(self.buffer.as_ref() as *const _ as *mut _) };
            let written = self.write_data(buf)?;
            let _ = self.buffer.split_to(written);
            if shutdown && !self.buffer.is_empty()
                || (self.buffer.len() > self.buffer_capacity)
            {
                return Ok(Async::NotReady);
            }
        }
        if shutdown {
            self.stream.shutdown()
        } else {
            Ok(Async::Ready(()))
        }
    }
}
