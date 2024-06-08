use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     MessageGetResponse,
     MessagePostResponse,
     MessagePutResponse,
     AuthDeleteResponse,
     AuthGetResponse,
     AuthGet_0Response,
     AuthPostResponse,
     AuthPutResponse,
     RegisterGetResponse,
     RegisterPostResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/auth$",
            r"^/v1/auth/$",
            r"^/v1/message$",
            r"^/v1/message/$",
            r"^/v1/register$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_AUTH: usize = 0;
    pub(crate) static ID_AUTH_: usize = 1;
    pub(crate) static ID_MESSAGE: usize = 2;
    pub(crate) static ID_MESSAGE_: usize = 3;
    lazy_static! {
        pub static ref REGEX_MESSAGE_: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/message/$")
                .expect("Unable to create regex for MESSAGE_");
    }
    pub(crate) static ID_REGISTER: usize = 4;
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // MessageGet - GET /message
            hyper::Method::GET if path.matched(paths::ID_MESSAGE) => {
                                let result = api_impl.message_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MessageGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for MESSAGE_GET_OK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                MessageGetResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MessagePost - POST /message
            hyper::Method::POST if path.matched(paths::ID_MESSAGE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MESSAGE_
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MESSAGE in set but failed match against \"{}\"", path, paths::REGEX_MESSAGE.as_str())
                    );

                let param_body = match percent_encoding::percent_decode(path_params["body"].as_bytes()).decode_utf8() {
                    Ok(param_body) => match param_body.parse::<models::Message>() {
                        Ok(param_body) => param_body,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter body: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["body"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.message_post(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MessagePostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // MessagePut - PUT /message/
            hyper::Method::PUT if path.matched(paths::ID_MESSAGE_) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_MESSAGE_
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MESSAGE_ in set but failed match against \"{}\"", path, paths::REGEX_MESSAGE_.as_str())
                    );

                let param_body = match percent_encoding::percent_decode(path_params["body"].as_bytes()).decode_utf8() {
                    Ok(param_body) => match param_body.parse::<models::Message>() {
                        Ok(param_body) => param_body,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter body: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["body"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_message_id = query_params.iter().filter(|e| e.0 == "MessageId").map(|e| e.1.clone())
                    .next();
                let param_message_id = match param_message_id {
                    Some(param_message_id) => {
                        let param_message_id =
                            <i32 as std::str::FromStr>::from_str
                                (&param_message_id);
                        match param_message_id {
                            Ok(param_message_id) => Some(param_message_id),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter MessageId - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter MessageId")),
                        }
                    },
                    None => None,
                };
                let param_message_id = match param_message_id {
                    Some(param_message_id) => param_message_id,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter MessageId"))
                        .expect("Unable to create Bad Request response for missing query parameter MessageId")),
                };

                                let result = api_impl.message_put(
                                            param_message_id,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MessagePutResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AuthDelete - DELETE /auth/
            hyper::Method::DELETE if path.matched(paths::ID_AUTH_) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_email = query_params.iter().filter(|e| e.0 == "email").map(|e| e.1.clone())
                    .next();
                let param_email = match param_email {
                    Some(param_email) => {
                        let param_email =
                            <String as std::str::FromStr>::from_str
                                (&param_email);
                        match param_email {
                            Ok(param_email) => Some(param_email),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter email - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter email")),
                        }
                    },
                    None => None,
                };
                let param_email = match param_email {
                    Some(param_email) => param_email,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter email"))
                        .expect("Unable to create Bad Request response for missing query parameter email")),
                };

                                let result = api_impl.auth_delete(
                                            param_email,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AuthDeleteResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                AuthDeleteResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AuthGet - GET /auth
            hyper::Method::GET if path.matched(paths::ID_AUTH) => {
                                let result = api_impl.auth_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AuthGetResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                AuthGetResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AuthGet_0 - GET /auth/
            hyper::Method::GET if path.matched(paths::ID_AUTH_) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_email = query_params.iter().filter(|e| e.0 == "email").map(|e| e.1.clone())
                    .next();
                let param_email = match param_email {
                    Some(param_email) => {
                        let param_email =
                            <String as std::str::FromStr>::from_str
                                (&param_email);
                        match param_email {
                            Ok(param_email) => Some(param_email),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter email - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter email")),
                        }
                    },
                    None => None,
                };
                let param_email = match param_email {
                    Some(param_email) => param_email,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter email"))
                        .expect("Unable to create Bad Request response for missing query parameter email")),
                };

                                let result = api_impl.auth_get(
                                            param_email,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AuthGet_0Response::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for AUTH_GET_OK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                AuthGet_0Response::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AuthPost - POST /auth
            hyper::Method::POST if path.matched(paths::ID_AUTH) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_AUTH
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE AUTH in set but failed match against \"{}\"", path, paths::REGEX_AUTH.as_str())
                    );

                let param_body = match percent_encoding::percent_decode(path_params["body"].as_bytes()).decode_utf8() {
                    Ok(param_body) => match param_body.parse::<models::User>() {
                        Ok(param_body) => param_body,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter body: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["body"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.auth_post(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AuthPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                AuthPostResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AuthPut - PUT /auth/
            hyper::Method::PUT if path.matched(paths::ID_AUTH_) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_AUTH_
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE AUTH_ in set but failed match against \"{}\"", path, paths::REGEX_AUTH_.as_str())
                    );

                let param_body = match percent_encoding::percent_decode(path_params["body"].as_bytes()).decode_utf8() {
                    Ok(param_body) => match param_body.parse::<models::User>() {
                        Ok(param_body) => param_body,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter body: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["body"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_email = query_params.iter().filter(|e| e.0 == "email").map(|e| e.1.clone())
                    .next();
                let param_email = match param_email {
                    Some(param_email) => {
                        let param_email =
                            <String as std::str::FromStr>::from_str
                                (&param_email);
                        match param_email {
                            Ok(param_email) => Some(param_email),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter email - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter email")),
                        }
                    },
                    None => None,
                };
                let param_email = match param_email {
                    Some(param_email) => param_email,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter email"))
                        .expect("Unable to create Bad Request response for missing query parameter email")),
                };

                                let result = api_impl.auth_put(
                                            param_email,
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AuthPutResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RegisterGet - GET /register
            hyper::Method::GET if path.matched(paths::ID_REGISTER) => {
                                let result = api_impl.register_get(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RegisterGetResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                RegisterGetResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RegisterPost - POST /register
            hyper::Method::POST if path.matched(paths::ID_REGISTER) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_REGISTER
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE REGISTER in set but failed match against \"{}\"", path, paths::REGEX_REGISTER.as_str())
                    );

                let param_body = match percent_encoding::percent_decode(path_params["body"].as_bytes()).decode_utf8() {
                    Ok(param_body) => match param_body.parse::<models::User>() {
                        Ok(param_body) => param_body,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter body: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["body"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.register_post(
                                            param_body,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RegisterPostResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                RegisterPostResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_AUTH) => method_not_allowed(),
            _ if path.matched(paths::ID_AUTH_) => method_not_allowed(),
            _ if path.matched(paths::ID_MESSAGE) => method_not_allowed(),
            _ if path.matched(paths::ID_MESSAGE_) => method_not_allowed(),
            _ if path.matched(paths::ID_REGISTER) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // MessageGet - GET /message
            hyper::Method::GET if path.matched(paths::ID_MESSAGE) => Some("MessageGet"),
            // MessagePost - POST /message
            hyper::Method::POST if path.matched(paths::ID_MESSAGE) => Some("MessagePost"),
            // MessagePut - PUT /message/
            hyper::Method::PUT if path.matched(paths::ID_MESSAGE_) => Some("MessagePut"),
            // AuthDelete - DELETE /auth/
            hyper::Method::DELETE if path.matched(paths::ID_AUTH_) => Some("AuthDelete"),
            // AuthGet - GET /auth
            hyper::Method::GET if path.matched(paths::ID_AUTH) => Some("AuthGet"),
            // AuthGet_0 - GET /auth/
            hyper::Method::GET if path.matched(paths::ID_AUTH_) => Some("AuthGet_0"),
            // AuthPost - POST /auth
            hyper::Method::POST if path.matched(paths::ID_AUTH) => Some("AuthPost"),
            // AuthPut - PUT /auth/
            hyper::Method::PUT if path.matched(paths::ID_AUTH_) => Some("AuthPut"),
            // RegisterGet - GET /register
            hyper::Method::GET if path.matched(paths::ID_REGISTER) => Some("RegisterGet"),
            // RegisterPost - POST /register
            hyper::Method::POST if path.matched(paths::ID_REGISTER) => Some("RegisterPost"),
            _ => None,
        }
    }
}
