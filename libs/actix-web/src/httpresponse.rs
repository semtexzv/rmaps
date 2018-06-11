//! Http response
use std::cell::UnsafeCell;
use std::collections::VecDeque;
use std::io::Write;
use std::rc::Rc;
use std::{fmt, mem, str};

use bytes::{BufMut, Bytes, BytesMut};
use cookie::{Cookie, CookieJar};
use futures::Stream;
use http::header::{self, HeaderName, HeaderValue};
use http::{Error as HttpError, HeaderMap, HttpTryFrom, StatusCode, Version};
use serde::Serialize;
use serde_json;

use body::Body;
use client::ClientResponse;
use error::Error;
use handler::Responder;
use header::{ContentEncoding, Header, IntoHeaderValue};
use httpmessage::HttpMessage;
use httprequest::HttpRequest;

/// max write buffer size 64k
pub(crate) const MAX_WRITE_BUFFER_SIZE: usize = 65_536;

/// Represents various types of connection
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConnectionType {
    /// Close connection after response
    Close,
    /// Keep connection alive after response
    KeepAlive,
    /// Connection is upgraded to different type
    Upgrade,
}

/// An HTTP Response
pub struct HttpResponse(
    Option<Box<InnerHttpResponse>>,
    Rc<UnsafeCell<HttpResponsePool>>,
);

impl Drop for HttpResponse {
    fn drop(&mut self) {
        if let Some(inner) = self.0.take() {
            HttpResponsePool::release(&self.1, inner)
        }
    }
}

impl HttpResponse {
    #[inline(always)]
    #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
    fn get_ref(&self) -> &InnerHttpResponse {
        self.0.as_ref().unwrap()
    }

    #[inline(always)]
    #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
    fn get_mut(&mut self) -> &mut InnerHttpResponse {
        self.0.as_mut().unwrap()
    }

    /// Create http response builder with specific status.
    #[inline]
    pub fn build(status: StatusCode) -> HttpResponseBuilder {
        HttpResponsePool::get(status)
    }

    /// Create http response builder
    #[inline]
    pub fn build_from<T: Into<HttpResponseBuilder>>(source: T) -> HttpResponseBuilder {
        source.into()
    }

    /// Constructs a response
    #[inline]
    pub fn new(status: StatusCode) -> HttpResponse {
        HttpResponsePool::with_body(status, Body::Empty)
    }

    /// Constructs a response with body
    #[inline]
    pub fn with_body<B: Into<Body>>(status: StatusCode, body: B) -> HttpResponse {
        HttpResponsePool::with_body(status, body.into())
    }

    /// Constructs a error response
    #[inline]
    pub fn from_error(error: Error) -> HttpResponse {
        let mut resp = error.as_response_error().error_response();
        resp.get_mut().error = Some(error);
        resp
    }

    /// Convert `HttpResponse` to a `HttpResponseBuilder`
    #[inline]
    pub fn into_builder(mut self) -> HttpResponseBuilder {
        let response = self.0.take();
        let pool = Some(Rc::clone(&self.1));

        HttpResponseBuilder {
            response,
            pool,
            err: None,
            cookies: None, // TODO: convert set-cookie headers
        }
    }

    /// The source `error` for this response
    #[inline]
    pub fn error(&self) -> Option<&Error> {
        self.get_ref().error.as_ref()
    }

    /// Get the HTTP version of this response
    #[inline]
    pub fn version(&self) -> Option<Version> {
        self.get_ref().version
    }

    /// Get the headers from the response
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.get_ref().headers
    }

    /// Get a mutable reference to the headers
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.get_mut().headers
    }

    /// Get the response status code
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.get_ref().status
    }

    /// Set the `StatusCode` for this response
    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        &mut self.get_mut().status
    }

    /// Get custom reason for the response
    #[inline]
    pub fn reason(&self) -> &str {
        if let Some(reason) = self.get_ref().reason {
            reason
        } else {
            self.get_ref()
                .status
                .canonical_reason()
                .unwrap_or("<unknown status code>")
        }
    }

    /// Set the custom reason for the response
    #[inline]
    pub fn set_reason(&mut self, reason: &'static str) -> &mut Self {
        self.get_mut().reason = Some(reason);
        self
    }

    /// Set connection type
    pub fn set_connection_type(&mut self, conn: ConnectionType) -> &mut Self {
        self.get_mut().connection_type = Some(conn);
        self
    }

    /// Connection upgrade status
    #[inline]
    pub fn upgrade(&self) -> bool {
        self.get_ref().connection_type == Some(ConnectionType::Upgrade)
    }

    /// Keep-alive status for this connection
    pub fn keep_alive(&self) -> Option<bool> {
        if let Some(ct) = self.get_ref().connection_type {
            match ct {
                ConnectionType::KeepAlive => Some(true),
                ConnectionType::Close | ConnectionType::Upgrade => Some(false),
            }
        } else {
            None
        }
    }

    /// is chunked encoding enabled
    #[inline]
    pub fn chunked(&self) -> Option<bool> {
        self.get_ref().chunked
    }

    /// Content encoding
    #[inline]
    pub fn content_encoding(&self) -> Option<ContentEncoding> {
        self.get_ref().encoding
    }

    /// Set content encoding
    pub fn set_content_encoding(&mut self, enc: ContentEncoding) -> &mut Self {
        self.get_mut().encoding = Some(enc);
        self
    }

    /// Get body os this response
    #[inline]
    pub fn body(&self) -> &Body {
        &self.get_ref().body
    }

    /// Set a body
    pub fn set_body<B: Into<Body>>(&mut self, body: B) {
        self.get_mut().body = body.into();
    }

    /// Set a body and return previous body value
    pub fn replace_body<B: Into<Body>>(&mut self, body: B) -> Body {
        mem::replace(&mut self.get_mut().body, body.into())
    }

    /// Size of response in bytes, excluding HTTP headers
    pub fn response_size(&self) -> u64 {
        self.get_ref().response_size
    }

    /// Set content encoding
    pub(crate) fn set_response_size(&mut self, size: u64) {
        self.get_mut().response_size = size;
    }

    /// Set write buffer capacity
    pub fn write_buffer_capacity(&self) -> usize {
        self.get_ref().write_capacity
    }

    /// Set write buffer capacity
    pub fn set_write_buffer_capacity(&mut self, cap: usize) {
        self.get_mut().write_capacity = cap;
    }
}

impl fmt::Debug for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = writeln!(
            f,
            "\nHttpResponse {:?} {}{}",
            self.get_ref().version,
            self.get_ref().status,
            self.get_ref().reason.unwrap_or("")
        );
        let _ = writeln!(f, "  encoding: {:?}", self.get_ref().encoding);
        let _ = writeln!(f, "  headers:");
        for (key, val) in self.get_ref().headers.iter() {
            let _ = writeln!(f, "    {:?}: {:?}", key, val);
        }
        res
    }
}

/// An HTTP response builder
///
/// This type can be used to construct an instance of `HttpResponse` through a
/// builder-like pattern.
pub struct HttpResponseBuilder {
    response: Option<Box<InnerHttpResponse>>,
    pool: Option<Rc<UnsafeCell<HttpResponsePool>>>,
    err: Option<HttpError>,
    cookies: Option<CookieJar>,
}

impl HttpResponseBuilder {
    /// Set HTTP status code of this response.
    #[inline]
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.status = status;
        }
        self
    }

    /// Set HTTP version of this response.
    ///
    /// By default response's http version depends on request's version.
    #[inline]
    pub fn version(&mut self, version: Version) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.version = Some(version);
        }
        self
    }

    /// Set a header.
    ///
    /// ```rust
    /// # extern crate actix_web;
    /// use actix_web::{HttpRequest, HttpResponse, Result, http};
    ///
    /// fn index(req: HttpRequest) -> Result<HttpResponse> {
    ///     Ok(HttpResponse::Ok()
    ///         .set(http::header::IfModifiedSince("Sun, 07 Nov 1994 08:48:37 GMT".parse()?))
    ///         .finish())
    /// }
    /// fn main() {}
    /// ```
    #[doc(hidden)]
    pub fn set<H: Header>(&mut self, hdr: H) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            match hdr.try_into() {
                Ok(value) => {
                    parts.headers.append(H::name(), value);
                }
                Err(e) => self.err = Some(e.into()),
            }
        }
        self
    }

    /// Set a header.
    ///
    /// ```rust
    /// # extern crate actix_web;
    /// use actix_web::{http, HttpRequest, HttpResponse};
    ///
    /// fn index(req: HttpRequest) -> HttpResponse {
    ///     HttpResponse::Ok()
    ///         .header("X-TEST", "value")
    ///         .header(http::header::CONTENT_TYPE, "application/json")
    ///         .finish()
    /// }
    /// fn main() {}
    /// ```
    pub fn header<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        HeaderName: HttpTryFrom<K>,
        V: IntoHeaderValue,
    {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            match HeaderName::try_from(key) {
                Ok(key) => match value.try_into() {
                    Ok(value) => {
                        parts.headers.append(key, value);
                    }
                    Err(e) => self.err = Some(e.into()),
                },
                Err(e) => self.err = Some(e.into()),
            };
        }
        self
    }

    /// Set the custom reason for the response.
    #[inline]
    pub fn reason(&mut self, reason: &'static str) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.reason = Some(reason);
        }
        self
    }

    /// Set content encoding.
    ///
    /// By default `ContentEncoding::Auto` is used, which automatically
    /// negotiates content encoding based on request's `Accept-Encoding`
    /// headers. To enforce specific encoding, use specific
    /// ContentEncoding` value.
    #[inline]
    pub fn content_encoding(&mut self, enc: ContentEncoding) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.encoding = Some(enc);
        }
        self
    }

    /// Set connection type
    #[inline]
    #[doc(hidden)]
    pub fn connection_type(&mut self, conn: ConnectionType) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.connection_type = Some(conn);
        }
        self
    }

    /// Set connection type to Upgrade
    #[inline]
    #[doc(hidden)]
    pub fn upgrade(&mut self) -> &mut Self {
        self.connection_type(ConnectionType::Upgrade)
    }

    /// Force close connection, even if it is marked as keep-alive
    #[inline]
    pub fn force_close(&mut self) -> &mut Self {
        self.connection_type(ConnectionType::Close)
    }

    /// Enables automatic chunked transfer encoding
    #[inline]
    pub fn chunked(&mut self) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.chunked = Some(true);
        }
        self
    }

    /// Force disable chunked encoding
    #[inline]
    pub fn no_chunking(&mut self) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.chunked = Some(false);
        }
        self
    }

    /// Set response content type
    #[inline]
    pub fn content_type<V>(&mut self, value: V) -> &mut Self
    where
        HeaderValue: HttpTryFrom<V>,
    {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            match HeaderValue::try_from(value) {
                Ok(value) => {
                    parts.headers.insert(header::CONTENT_TYPE, value);
                }
                Err(e) => self.err = Some(e.into()),
            };
        }
        self
    }

    /// Set content length
    #[inline]
    pub fn content_length(&mut self, len: u64) -> &mut Self {
        let mut wrt = BytesMut::new().writer();
        let _ = write!(wrt, "{}", len);
        self.header(header::CONTENT_LENGTH, wrt.get_mut().take().freeze())
    }

    /// Set a cookie
    ///
    /// ```rust
    /// # extern crate actix_web;
    /// use actix_web::{http, HttpRequest, HttpResponse, Result};
    ///
    /// fn index(req: HttpRequest) -> HttpResponse {
    ///     HttpResponse::Ok()
    ///         .cookie(
    ///             http::Cookie::build("name", "value")
    ///                 .domain("www.rust-lang.org")
    ///                 .path("/")
    ///                 .secure(true)
    ///                 .http_only(true)
    ///                 .finish())
    ///         .finish()
    /// }
    /// fn main() {}
    /// ```
    pub fn cookie<'c>(&mut self, cookie: Cookie<'c>) -> &mut Self {
        if self.cookies.is_none() {
            let mut jar = CookieJar::new();
            jar.add(cookie.into_owned());
            self.cookies = Some(jar)
        } else {
            self.cookies.as_mut().unwrap().add(cookie.into_owned());
        }
        self
    }

    /// Remove cookie, cookie has to be cookie from `HttpRequest::cookies()`
    /// method.
    pub fn del_cookie<'a>(&mut self, cookie: &Cookie<'a>) -> &mut Self {
        {
            if self.cookies.is_none() {
                self.cookies = Some(CookieJar::new())
            }
            let jar = self.cookies.as_mut().unwrap();
            let cookie = cookie.clone().into_owned();
            jar.add_original(cookie.clone());
            jar.remove(cookie);
        }
        self
    }

    /// This method calls provided closure with builder reference if value is
    /// true.
    pub fn if_true<F>(&mut self, value: bool, f: F) -> &mut Self
    where
        F: FnOnce(&mut HttpResponseBuilder),
    {
        if value {
            f(self);
        }
        self
    }

    /// This method calls provided closure with builder reference if value is
    /// Some.
    pub fn if_some<T, F>(&mut self, value: Option<T>, f: F) -> &mut Self
    where
        F: FnOnce(T, &mut HttpResponseBuilder),
    {
        if let Some(val) = value {
            f(val, self);
        }
        self
    }

    /// Set write buffer capacity
    ///
    /// This parameter makes sense only for streaming response
    /// or actor. If write buffer reaches specified capacity, stream or actor
    /// get paused.
    ///
    /// Default write buffer capacity is 64kb
    pub fn write_buffer_capacity(&mut self, cap: usize) -> &mut Self {
        if let Some(parts) = parts(&mut self.response, &self.err) {
            parts.write_capacity = cap;
        }
        self
    }

    /// Set a body and generate `HttpResponse`.
    ///
    /// `HttpResponseBuilder` can not be used after this call.
    pub fn body<B: Into<Body>>(&mut self, body: B) -> HttpResponse {
        if let Some(e) = self.err.take() {
            return Error::from(e).into();
        }
        let mut response = self.response.take().expect("cannot reuse response builder");
        if let Some(ref jar) = self.cookies {
            for cookie in jar.delta() {
                match HeaderValue::from_str(&cookie.to_string()) {
                    Ok(val) => response.headers.append(header::SET_COOKIE, val),
                    Err(e) => return Error::from(e).into(),
                };
            }
        }
        response.body = body.into();
        HttpResponse(Some(response), self.pool.take().unwrap())
    }

    #[inline]
    /// Set a streaming body and generate `HttpResponse`.
    ///
    /// `HttpResponseBuilder` can not be used after this call.
    pub fn streaming<S, E>(&mut self, stream: S) -> HttpResponse
    where
        S: Stream<Item = Bytes, Error = E> + 'static,
        E: Into<Error>,
    {
        self.body(Body::Streaming(Box::new(stream.map_err(|e| e.into()))))
    }

    /// Set a json body and generate `HttpResponse`
    ///
    /// `HttpResponseBuilder` can not be used after this call.
    pub fn json<T: Serialize>(&mut self, value: T) -> HttpResponse {
        match serde_json::to_string(&value) {
            Ok(body) => {
                let contains = if let Some(parts) = parts(&mut self.response, &self.err)
                {
                    parts.headers.contains_key(header::CONTENT_TYPE)
                } else {
                    true
                };
                if !contains {
                    self.header(header::CONTENT_TYPE, "application/json");
                }

                self.body(body)
            }
            Err(e) => Error::from(e).into(),
        }
    }

    #[inline]
    /// Set an empty body and generate `HttpResponse`
    ///
    /// `HttpResponseBuilder` can not be used after this call.
    pub fn finish(&mut self) -> HttpResponse {
        self.body(Body::Empty)
    }

    /// This method construct new `HttpResponseBuilder`
    pub fn take(&mut self) -> HttpResponseBuilder {
        HttpResponseBuilder {
            response: self.response.take(),
            pool: self.pool.take(),
            err: self.err.take(),
            cookies: self.cookies.take(),
        }
    }
}

#[inline]
#[cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]
fn parts<'a>(
    parts: &'a mut Option<Box<InnerHttpResponse>>, err: &Option<HttpError>,
) -> Option<&'a mut Box<InnerHttpResponse>> {
    if err.is_some() {
        return None;
    }
    parts.as_mut()
}

/// Helper converters
impl<I: Into<HttpResponse>, E: Into<Error>> From<Result<I, E>> for HttpResponse {
    fn from(res: Result<I, E>) -> Self {
        match res {
            Ok(val) => val.into(),
            Err(err) => err.into().into(),
        }
    }
}

impl From<HttpResponseBuilder> for HttpResponse {
    fn from(mut builder: HttpResponseBuilder) -> Self {
        builder.finish()
    }
}

impl Responder for HttpResponseBuilder {
    type Item = HttpResponse;
    type Error = Error;

    #[inline]
    fn respond_to<S>(mut self, _: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(self.finish())
    }
}

impl From<&'static str> for HttpResponse {
    fn from(val: &'static str) -> Self {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(val)
    }
}

impl Responder for &'static str {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(self))
    }
}

impl From<&'static [u8]> for HttpResponse {
    fn from(val: &'static [u8]) -> Self {
        HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(val)
    }
}

impl Responder for &'static [u8] {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("application/octet-stream")
            .body(self))
    }
}

impl From<String> for HttpResponse {
    fn from(val: String) -> Self {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(val)
    }
}

impl Responder for String {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(self))
    }
}

impl<'a> From<&'a String> for HttpResponse {
    fn from(val: &'a String) -> Self {
        HttpResponse::build(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(val)
    }
}

impl<'a> Responder for &'a String {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(self))
    }
}

impl From<Bytes> for HttpResponse {
    fn from(val: Bytes) -> Self {
        HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(val)
    }
}

impl Responder for Bytes {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("application/octet-stream")
            .body(self))
    }
}

impl From<BytesMut> for HttpResponse {
    fn from(val: BytesMut) -> Self {
        HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(val)
    }
}

impl Responder for BytesMut {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        Ok(req
            .build_response(StatusCode::OK)
            .content_type("application/octet-stream")
            .body(self))
    }
}

/// Create `HttpResponseBuilder` from `ClientResponse`
///
/// It is useful for proxy response. This implementation
/// copies all responses's headers and status.
impl<'a> From<&'a ClientResponse> for HttpResponseBuilder {
    fn from(resp: &'a ClientResponse) -> HttpResponseBuilder {
        let mut builder = HttpResponse::build(resp.status());
        for (key, value) in resp.headers() {
            builder.header(key.clone(), value.clone());
        }
        builder
    }
}

impl<'a, S> From<&'a HttpRequest<S>> for HttpResponseBuilder {
    fn from(req: &'a HttpRequest<S>) -> HttpResponseBuilder {
        if let Some(router) = req.router() {
            router
                .server_settings()
                .get_response_builder(StatusCode::OK)
        } else {
            HttpResponse::Ok()
        }
    }
}

#[derive(Debug)]
struct InnerHttpResponse {
    version: Option<Version>,
    headers: HeaderMap,
    status: StatusCode,
    reason: Option<&'static str>,
    body: Body,
    chunked: Option<bool>,
    encoding: Option<ContentEncoding>,
    connection_type: Option<ConnectionType>,
    write_capacity: usize,
    response_size: u64,
    error: Option<Error>,
}

impl InnerHttpResponse {
    #[inline]
    fn new(status: StatusCode, body: Body) -> InnerHttpResponse {
        InnerHttpResponse {
            status,
            body,
            version: None,
            headers: HeaderMap::with_capacity(16),
            reason: None,
            chunked: None,
            encoding: None,
            connection_type: None,
            response_size: 0,
            write_capacity: MAX_WRITE_BUFFER_SIZE,
            error: None,
        }
    }
}

/// Internal use only! unsafe
pub(crate) struct HttpResponsePool(VecDeque<Box<InnerHttpResponse>>);

thread_local!(static POOL: Rc<UnsafeCell<HttpResponsePool>> = HttpResponsePool::pool());

impl HttpResponsePool {
    pub fn pool() -> Rc<UnsafeCell<HttpResponsePool>> {
        Rc::new(UnsafeCell::new(HttpResponsePool(VecDeque::with_capacity(
            128,
        ))))
    }

    #[inline]
    pub fn get_builder(
        pool: &Rc<UnsafeCell<HttpResponsePool>>, status: StatusCode,
    ) -> HttpResponseBuilder {
        let p = unsafe { &mut *pool.as_ref().get() };
        if let Some(mut msg) = p.0.pop_front() {
            msg.status = status;
            HttpResponseBuilder {
                response: Some(msg),
                pool: Some(Rc::clone(pool)),
                err: None,
                cookies: None,
            }
        } else {
            let msg = Box::new(InnerHttpResponse::new(status, Body::Empty));
            HttpResponseBuilder {
                response: Some(msg),
                pool: Some(Rc::clone(pool)),
                err: None,
                cookies: None,
            }
        }
    }

    #[inline]
    pub fn get_response(
        pool: &Rc<UnsafeCell<HttpResponsePool>>, status: StatusCode, body: Body,
    ) -> HttpResponse {
        let p = unsafe { &mut *pool.as_ref().get() };
        if let Some(mut msg) = p.0.pop_front() {
            msg.status = status;
            msg.body = body;
            HttpResponse(Some(msg), Rc::clone(pool))
        } else {
            let msg = Box::new(InnerHttpResponse::new(status, body));
            HttpResponse(Some(msg), Rc::clone(pool))
        }
    }

    #[inline]
    fn get(status: StatusCode) -> HttpResponseBuilder {
        POOL.with(|pool| HttpResponsePool::get_builder(pool, status))
    }

    #[inline]
    fn with_body(status: StatusCode, body: Body) -> HttpResponse {
        POOL.with(|pool| HttpResponsePool::get_response(pool, status, body))
    }

    #[inline(always)]
    #[cfg_attr(feature = "cargo-clippy", allow(boxed_local, inline_always))]
    fn release(
        pool: &Rc<UnsafeCell<HttpResponsePool>>, mut inner: Box<InnerHttpResponse>,
    ) {
        let pool = unsafe { &mut *pool.as_ref().get() };
        if pool.0.len() < 128 {
            inner.headers.clear();
            inner.version = None;
            inner.chunked = None;
            inner.reason = None;
            inner.encoding = None;
            inner.connection_type = None;
            inner.response_size = 0;
            inner.error = None;
            inner.write_capacity = MAX_WRITE_BUFFER_SIZE;
            pool.0.push_front(inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use body::Binary;
    use http;
    use http::header::{HeaderValue, CONTENT_TYPE, COOKIE};
    use http::{Method, Uri};
    use std::str::FromStr;
    use time::Duration;

    #[test]
    fn test_debug() {
        let resp = HttpResponse::Ok()
            .header(COOKIE, HeaderValue::from_static("cookie1=value1; "))
            .header(COOKIE, HeaderValue::from_static("cookie2=value2; "))
            .finish();
        let dbg = format!("{:?}", resp);
        assert!(dbg.contains("HttpResponse"));
    }

    #[test]
    fn test_response_cookies() {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_static("cookie1=value1"));
        headers.insert(COOKIE, HeaderValue::from_static("cookie2=value2"));

        let req = HttpRequest::new(
            Method::GET,
            Uri::from_str("/").unwrap(),
            Version::HTTP_11,
            headers,
            None,
        );
        let cookies = req.cookies().unwrap();

        let resp = HttpResponse::Ok()
            .cookie(
                http::Cookie::build("name", "value")
                    .domain("www.rust-lang.org")
                    .path("/test")
                    .http_only(true)
                    .max_age(Duration::days(1))
                    .finish(),
            )
            .del_cookie(&cookies[0])
            .finish();

        let mut val: Vec<_> = resp
            .headers()
            .get_all("Set-Cookie")
            .iter()
            .map(|v| v.to_str().unwrap().to_owned())
            .collect();
        val.sort();
        assert!(val[0].starts_with("cookie2=; Max-Age=0;"));
        assert_eq!(
            val[1],
            "name=value; HttpOnly; Path=/test; Domain=www.rust-lang.org; Max-Age=86400"
        );
    }

    #[test]
    fn test_basic_builder() {
        let resp = HttpResponse::Ok()
            .header("X-TEST", "value")
            .version(Version::HTTP_10)
            .finish();
        assert_eq!(resp.version(), Some(Version::HTTP_10));
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_upgrade() {
        let resp = HttpResponse::build(StatusCode::OK).upgrade().finish();
        assert!(resp.upgrade())
    }

    #[test]
    fn test_force_close() {
        let resp = HttpResponse::build(StatusCode::OK).force_close().finish();
        assert!(!resp.keep_alive().unwrap())
    }

    #[test]
    fn test_content_type() {
        let resp = HttpResponse::build(StatusCode::OK)
            .content_type("text/plain")
            .body(Body::Empty);
        assert_eq!(resp.headers().get(CONTENT_TYPE).unwrap(), "text/plain")
    }

    #[test]
    fn test_content_encoding() {
        let resp = HttpResponse::build(StatusCode::OK).finish();
        assert_eq!(resp.content_encoding(), None);

        #[cfg(feature = "brotli")]
        {
            let resp = HttpResponse::build(StatusCode::OK)
                .content_encoding(ContentEncoding::Br)
                .finish();
            assert_eq!(resp.content_encoding(), Some(ContentEncoding::Br));
        }

        let resp = HttpResponse::build(StatusCode::OK)
            .content_encoding(ContentEncoding::Gzip)
            .finish();
        assert_eq!(resp.content_encoding(), Some(ContentEncoding::Gzip));
    }

    #[test]
    fn test_json() {
        let resp = HttpResponse::build(StatusCode::OK).json(vec!["v1", "v2", "v3"]);
        let ct = resp.headers().get(CONTENT_TYPE).unwrap();
        assert_eq!(ct, HeaderValue::from_static("application/json"));
        assert_eq!(
            *resp.body(),
            Body::from(Bytes::from_static(b"[\"v1\",\"v2\",\"v3\"]"))
        );
    }

    #[test]
    fn test_json_ct() {
        let resp = HttpResponse::build(StatusCode::OK)
            .header(CONTENT_TYPE, "text/json")
            .json(vec!["v1", "v2", "v3"]);
        let ct = resp.headers().get(CONTENT_TYPE).unwrap();
        assert_eq!(ct, HeaderValue::from_static("text/json"));
        assert_eq!(
            *resp.body(),
            Body::from(Bytes::from_static(b"[\"v1\",\"v2\",\"v3\"]"))
        );
    }

    impl Body {
        pub(crate) fn bin_ref(&self) -> &Binary {
            match *self {
                Body::Binary(ref bin) => bin,
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_into_response() {
        let req = HttpRequest::default();

        let resp: HttpResponse = "test".into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from("test"));

        let resp: HttpResponse = "test".respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from("test"));

        let resp: HttpResponse = b"test".as_ref().into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(b"test".as_ref()));

        let resp: HttpResponse = b"test".as_ref().respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(b"test".as_ref()));

        let resp: HttpResponse = "test".to_owned().into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from("test".to_owned()));

        let resp: HttpResponse = "test".to_owned().respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from("test".to_owned()));

        let resp: HttpResponse = (&"test".to_owned()).into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(&"test".to_owned()));

        let resp: HttpResponse = (&"test".to_owned()).respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/plain; charset=utf-8")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(&"test".to_owned()));

        let b = Bytes::from_static(b"test");
        let resp: HttpResponse = b.into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.body().bin_ref(),
            &Binary::from(Bytes::from_static(b"test"))
        );

        let b = Bytes::from_static(b"test");
        let resp: HttpResponse = b.respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.body().bin_ref(),
            &Binary::from(Bytes::from_static(b"test"))
        );

        let b = BytesMut::from("test");
        let resp: HttpResponse = b.into();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(BytesMut::from("test")));

        let b = BytesMut::from("test");
        let resp: HttpResponse = b.respond_to(&req).ok().unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/octet-stream")
        );
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().bin_ref(), &Binary::from(BytesMut::from("test")));
    }

    #[test]
    fn test_into_builder() {
        let resp: HttpResponse = "test".into();
        assert_eq!(resp.status(), StatusCode::OK);

        let mut builder = resp.into_builder();
        let resp = builder.status(StatusCode::BAD_REQUEST).finish();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
