use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str;

use bytes::Bytes;
use encoding::all::UTF_8;
use encoding::types::{DecoderTrap, Encoding};
use futures::{Async, Future, Poll};
use mime::Mime;
use serde::de::{self, DeserializeOwned};
use serde_urlencoded;

use de::PathDeserializer;
use error::{Error, ErrorBadRequest, ErrorNotFound};
use handler::{AsyncResult, FromRequest};
use httpmessage::{HttpMessage, MessageBody, UrlEncoded};
use httprequest::HttpRequest;

/// Extract typed information from the request's path.
///
/// ## Example
///
/// ```rust
/// # extern crate bytes;
/// # extern crate actix_web;
/// # extern crate futures;
/// use actix_web::{App, Path, Result, http};
///
/// /// extract path info from "/{username}/{count}/index.html" url
/// /// {username} - deserializes to a String
/// /// {count} -  - deserializes to a u32
/// fn index(info: Path<(String, u32)>) -> Result<String> {
///     Ok(format!("Welcome {}! {}", info.0, info.1))
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/{username}/{count}/index.html",              // <- define path parameters
///        |r| r.method(http::Method::GET).with(index));  // <- use `with` extractor
/// }
/// ```
///
/// It is possible to extract path information to a specific type that
/// implements `Deserialize` trait from *serde*.
///
/// ```rust
/// # extern crate bytes;
/// # extern crate actix_web;
/// # extern crate futures;
/// #[macro_use] extern crate serde_derive;
/// use actix_web::{App, Path, Result, http};
///
/// #[derive(Deserialize)]
/// struct Info {
///     username: String,
/// }
///
/// /// extract path info using serde
/// fn index(info: Path<Info>) -> Result<String> {
///     Ok(format!("Welcome {}!", info.username))
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/{username}/index.html",                // <- define path parameters
///        |r| r.method(http::Method::GET).with(index));  // <- use `with` extractor
/// }
/// ```
pub struct Path<T> {
    inner: T,
}

impl<T> AsRef<T> for Path<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for Path<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Path<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T, S> FromRequest<S> for Path<T>
where
    T: DeserializeOwned,
{
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        let req = req.clone();
        de::Deserialize::deserialize(PathDeserializer::new(&req))
            .map_err(ErrorNotFound)
            .map(|inner| Path { inner })
    }
}

/// Extract typed information from from the request's query.
///
/// ## Example
///
/// ```rust
/// # extern crate bytes;
/// # extern crate actix_web;
/// # extern crate futures;
/// #[macro_use] extern crate serde_derive;
/// use actix_web::{App, Query, http};
///
/// #[derive(Deserialize)]
/// struct Info {
///     username: String,
/// }
///
/// // use `with` extractor for query info
/// // this handler get called only if request's query contains `username` field
/// fn index(info: Query<Info>) -> String {
///     format!("Welcome {}!", info.username)
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/index.html",
///        |r| r.method(http::Method::GET).with(index)); // <- use `with` extractor
/// }
/// ```
pub struct Query<T>(T);

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Query<T> {
    /// Deconstruct to a inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, S> FromRequest<S> for Query<T>
where
    T: de::DeserializeOwned,
{
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        let req = req.clone();
        serde_urlencoded::from_str::<T>(req.query_string())
            .map_err(|e| e.into())
            .map(Query)
    }
}

/// Extract typed information from the request's body.
///
/// To extract typed information from request's body, the type `T` must
/// implement the `Deserialize` trait from *serde*.
///
/// [**FormConfig**](dev/struct.FormConfig.html) allows to configure extraction
/// process.
///
/// ## Example
///
/// ```rust
/// # extern crate actix_web;
/// #[macro_use] extern crate serde_derive;
/// use actix_web::{App, Form, Result};
///
/// #[derive(Deserialize)]
/// struct FormData {
///     username: String,
/// }
///
/// /// extract form data using serde
/// /// this handler get called only if content type is *x-www-form-urlencoded*
/// /// and content of the request could be deserialized to a `FormData` struct
/// fn index(form: Form<FormData>) -> Result<String> {
///     Ok(format!("Welcome {}!", form.username))
/// }
/// # fn main() {}
/// ```
pub struct Form<T>(pub T);

impl<T> Form<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned + 'static,
    S: 'static,
{
    type Config = FormConfig;
    type Result = Box<Future<Item = Self, Error = Error>>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, cfg: &Self::Config) -> Self::Result {
        Box::new(
            UrlEncoded::new(req.clone())
                .limit(cfg.limit)
                .from_err()
                .map(Form),
        )
    }
}

/// Form extractor configuration
///
/// ```rust
/// # extern crate actix_web;
/// #[macro_use] extern crate serde_derive;
/// use actix_web::{App, Form, Result, http};
///
/// #[derive(Deserialize)]
/// struct FormData {
///     username: String,
/// }
///
/// /// extract form data using serde.
/// /// custom configuration is used for this handler, max payload size is 4k
/// fn index(form: Form<FormData>) -> Result<String> {
///     Ok(format!("Welcome {}!", form.username))
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/index.html", |r| {
///            r.method(http::Method::GET)
///              .with(index)
///              .limit(4096);} // <- change form extractor configuration
///     );
/// }
/// ```
pub struct FormConfig {
    limit: usize,
}

impl FormConfig {
    /// Change max size of payload. By default max size is 256Kb
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        FormConfig { limit: 262_144 }
    }
}

/// Request payload extractor.
///
/// Loads request's payload and construct Bytes instance.
///
/// [**PayloadConfig**](dev/struct.PayloadConfig.html) allows to configure
/// extraction process.
///
/// ## Example
///
/// ```rust
/// extern crate bytes;
/// # extern crate actix_web;
/// use actix_web::{http, App, Result};
///
/// /// extract text data from request
/// fn index(body: bytes::Bytes) -> Result<String> {
///     Ok(format!("Body {:?}!", body))
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/index.html", |r|
///            r.method(http::Method::GET).with(index));
/// }
/// ```
impl<S: 'static> FromRequest<S> for Bytes {
    type Config = PayloadConfig;
    type Result = Result<Box<Future<Item = Self, Error = Error>>, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, cfg: &Self::Config) -> Self::Result {
        // check content-type
        cfg.check_mimetype(req)?;

        Ok(Box::new(
            MessageBody::new(req.clone()).limit(cfg.limit).from_err(),
        ))
    }
}

/// Extract text information from the request's body.
///
/// Text extractor automatically decode body according to the request's charset.
///
/// [**PayloadConfig**](dev/struct.PayloadConfig.html) allows to configure
/// extraction process.
///
/// ## Example
///
/// ```rust
/// # extern crate actix_web;
/// use actix_web::{http, App, Result};
///
/// /// extract text data from request
/// fn index(body: String) -> Result<String> {
///     Ok(format!("Body {}!", body))
/// }
///
/// fn main() {
///     let app = App::new().resource(
///        "/index.html", |r| {
///            r.method(http::Method::GET)
///                .with(index)   // <- register handler with extractor params
///                .limit(4096);  // <- limit size of the payload
///        });
/// }
/// ```
impl<S: 'static> FromRequest<S> for String {
    type Config = PayloadConfig;
    type Result = Result<Box<Future<Item = String, Error = Error>>, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, cfg: &Self::Config) -> Self::Result {
        // check content-type
        cfg.check_mimetype(req)?;

        // check charset
        let encoding = req.encoding()?;

        Ok(Box::new(
            MessageBody::new(req.clone())
                .limit(cfg.limit)
                .from_err()
                .and_then(move |body| {
                    let enc: *const Encoding = encoding as *const Encoding;
                    if enc == UTF_8 {
                        Ok(str::from_utf8(body.as_ref())
                            .map_err(|_| ErrorBadRequest("Can not decode body"))?
                            .to_owned())
                    } else {
                        Ok(encoding
                            .decode(&body, DecoderTrap::Strict)
                            .map_err(|_| ErrorBadRequest("Can not decode body"))?)
                    }
                }),
        ))
    }
}

/// Payload configuration for request's payload.
pub struct PayloadConfig {
    limit: usize,
    mimetype: Option<Mime>,
}

impl PayloadConfig {
    /// Change max size of payload. By default max size is 256Kb
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Set required mime-type of the request. By default mime type is not
    /// enforced.
    pub fn mimetype(&mut self, mt: Mime) -> &mut Self {
        self.mimetype = Some(mt);
        self
    }

    fn check_mimetype<S>(&self, req: &HttpRequest<S>) -> Result<(), Error> {
        // check content-type
        if let Some(ref mt) = self.mimetype {
            match req.mime_type() {
                Ok(Some(ref req_mt)) => {
                    if mt != req_mt {
                        return Err(ErrorBadRequest("Unexpected Content-Type"));
                    }
                }
                Ok(None) => {
                    return Err(ErrorBadRequest("Content-Type is expected"));
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
        Ok(())
    }
}

impl Default for PayloadConfig {
    fn default() -> Self {
        PayloadConfig {
            limit: 262_144,
            mimetype: None,
        }
    }
}

macro_rules! tuple_from_req ({$fut_type:ident, $(($n:tt, $T:ident)),+} => {

    /// FromRequest implementation for tuple
    impl<S, $($T: FromRequest<S> + 'static),+> FromRequest<S> for ($($T,)+)
    where
        S: 'static,
    {
        type Config = ($($T::Config,)+);
        type Result = Box<Future<Item = ($($T,)+), Error = Error>>;

        fn from_request(req: &HttpRequest<S>, cfg: &Self::Config) -> Self::Result {
            Box::new($fut_type {
                s: PhantomData,
                items: <($(Option<$T>,)+)>::default(),
                futs: ($(Some($T::from_request(req, &cfg.$n).into()),)+),
            })
        }
    }

    struct $fut_type<S, $($T: FromRequest<S>),+>
    where
        S: 'static,
    {
        s: PhantomData<S>,
        items: ($(Option<$T>,)+),
        futs: ($(Option<AsyncResult<$T>>,)+),
    }

    impl<S, $($T: FromRequest<S>),+> Future for $fut_type<S, $($T),+>
    where
        S: 'static,
    {
        type Item = ($($T,)+);
        type Error = Error;

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            let mut ready = true;

            $(
                if self.futs.$n.is_some() {
                    match self.futs.$n.as_mut().unwrap().poll() {
                        Ok(Async::Ready(item)) => {
                            self.items.$n = Some(item);
                            self.futs.$n.take();
                        }
                        Ok(Async::NotReady) => ready = false,
                        Err(e) => return Err(e),
                    }
                }
            )+

                if ready {
                    Ok(Async::Ready(
                        ($(self.items.$n.take().unwrap(),)+)
                    ))
                } else {
                    Ok(Async::NotReady)
                }
        }
    }
});

tuple_from_req!(TupleFromRequest1, (0, A));
tuple_from_req!(TupleFromRequest2, (0, A), (1, B));
tuple_from_req!(TupleFromRequest3, (0, A), (1, B), (2, C));
tuple_from_req!(TupleFromRequest4, (0, A), (1, B), (2, C), (3, D));
tuple_from_req!(TupleFromRequest5, (0, A), (1, B), (2, C), (3, D), (4, E));
tuple_from_req!(
    TupleFromRequest6,
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F)
);
tuple_from_req!(
    TupleFromRequest7,
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G)
);
tuple_from_req!(
    TupleFromRequest8,
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H)
);
tuple_from_req!(
    TupleFromRequest9,
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H),
    (8, I)
);

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use futures::{Async, Future};
    use http::header;
    use mime;
    use resource::ResourceHandler;
    use router::{Resource, Router};
    use server::ServerSettings;
    use test::TestRequest;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Info {
        hello: String,
    }

    #[test]
    fn test_bytes() {
        let cfg = PayloadConfig::default();
        let mut req = TestRequest::with_header(header::CONTENT_LENGTH, "11").finish();
        req.payload_mut()
            .unread_data(Bytes::from_static(b"hello=world"));

        match Bytes::from_request(&req, &cfg).unwrap().poll().unwrap() {
            Async::Ready(s) => {
                assert_eq!(s, Bytes::from_static(b"hello=world"));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_string() {
        let cfg = PayloadConfig::default();
        let mut req = TestRequest::with_header(header::CONTENT_LENGTH, "11").finish();
        req.payload_mut()
            .unread_data(Bytes::from_static(b"hello=world"));

        match String::from_request(&req, &cfg).unwrap().poll().unwrap() {
            Async::Ready(s) => {
                assert_eq!(s, "hello=world");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_form() {
        let mut req = TestRequest::with_header(
            header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        ).header(header::CONTENT_LENGTH, "11")
            .finish();
        req.payload_mut()
            .unread_data(Bytes::from_static(b"hello=world"));

        let mut cfg = FormConfig::default();
        cfg.limit(4096);
        match Form::<Info>::from_request(&req, &cfg).poll().unwrap() {
            Async::Ready(s) => {
                assert_eq!(s.hello, "world");
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_payload_config() {
        let req = HttpRequest::default();
        let mut cfg = PayloadConfig::default();
        cfg.mimetype(mime::APPLICATION_JSON);
        assert!(cfg.check_mimetype(&req).is_err());

        let req = TestRequest::with_header(
            header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        ).finish();
        assert!(cfg.check_mimetype(&req).is_err());

        let req =
            TestRequest::with_header(header::CONTENT_TYPE, "application/json").finish();
        assert!(cfg.check_mimetype(&req).is_ok());
    }

    #[derive(Deserialize)]
    struct MyStruct {
        key: String,
        value: String,
    }

    #[derive(Deserialize)]
    struct Id {
        id: String,
    }

    #[derive(Deserialize)]
    struct Test2 {
        key: String,
        value: u32,
    }

    #[test]
    fn test_request_extract() {
        let mut req = TestRequest::with_uri("/name/user1/?id=test").finish();

        let mut resource = ResourceHandler::<()>::default();
        resource.name("index");
        let mut routes = Vec::new();
        routes.push((Resource::new("index", "/{key}/{value}/"), Some(resource)));
        let (router, _) = Router::new("", ServerSettings::default(), routes);
        assert!(router.recognize(&mut req).is_some());

        let s = Path::<MyStruct>::from_request(&req, &()).unwrap();
        assert_eq!(s.key, "name");
        assert_eq!(s.value, "user1");

        let s = Path::<(String, String)>::from_request(&req, &()).unwrap();
        assert_eq!(s.0, "name");
        assert_eq!(s.1, "user1");

        let s = Query::<Id>::from_request(&req, &()).unwrap();
        assert_eq!(s.id, "test");

        let mut req = TestRequest::with_uri("/name/32/").finish();
        assert!(router.recognize(&mut req).is_some());

        let s = Path::<Test2>::from_request(&req, &()).unwrap();
        assert_eq!(s.as_ref().key, "name");
        assert_eq!(s.value, 32);

        let s = Path::<(String, u8)>::from_request(&req, &()).unwrap();
        assert_eq!(s.0, "name");
        assert_eq!(s.1, 32);

        let res = Path::<Vec<String>>::extract(&req).unwrap();
        assert_eq!(res[0], "name".to_owned());
        assert_eq!(res[1], "32".to_owned());
    }

    #[test]
    fn test_extract_path_single() {
        let mut resource = ResourceHandler::<()>::default();
        resource.name("index");
        let mut routes = Vec::new();
        routes.push((Resource::new("index", "/{value}/"), Some(resource)));
        let (router, _) = Router::new("", ServerSettings::default(), routes);

        let mut req = TestRequest::with_uri("/32/").finish();
        assert!(router.recognize(&mut req).is_some());

        assert_eq!(*Path::<i8>::from_request(&mut req, &()).unwrap(), 32);
    }

    #[test]
    fn test_tuple_extract() {
        let mut req = TestRequest::with_uri("/name/user1/?id=test").finish();

        let mut resource = ResourceHandler::<()>::default();
        resource.name("index");
        let mut routes = Vec::new();
        routes.push((Resource::new("index", "/{key}/{value}/"), Some(resource)));
        let (router, _) = Router::new("", ServerSettings::default(), routes);
        assert!(router.recognize(&mut req).is_some());

        let res = match <(Path<(String, String)>,)>::extract(&req).poll() {
            Ok(Async::Ready(res)) => res,
            _ => panic!("error"),
        };
        assert_eq!((res.0).0, "name");
        assert_eq!((res.0).1, "user1");

        let res = match <(Path<(String, String)>, Path<(String, String)>)>::extract(&req)
            .poll()
        {
            Ok(Async::Ready(res)) => res,
            _ => panic!("error"),
        };
        assert_eq!((res.0).0, "name");
        assert_eq!((res.0).1, "user1");
        assert_eq!((res.1).0, "name");
        assert_eq!((res.1).1, "user1");
    }
}
