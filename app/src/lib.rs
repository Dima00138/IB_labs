#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/v1";
pub const API_VERSION: &str = "1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum MessageGetResponse {
    /// OK
    OK
    (Vec<models::Message>)
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessagePostResponse {
    /// OK
    OK
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessagePutResponse {
    /// OK
    OK
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AuthDeleteResponse {
    /// OK
    OK
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AuthGetResponse {
    /// OK
    OK
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AuthGet_0Response {
    /// OK
    OK
    (models::User)
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum AuthPostResponse {
    /// OK
    OK
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AuthPutResponse {
    /// OK
    OK
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RegisterGetResponse {
    /// OK
    OK
    ,
    /// Not Found
    NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RegisterPostResponse {
    /// OK
    OK
    ,
    /// Not Found
    NotFound
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn message_get(
        &self,
        context: &C) -> Result<MessageGetResponse, ApiError>;

    async fn message_post(
        &self,
        body: models::Message,
        context: &C) -> Result<MessagePostResponse, ApiError>;

    async fn message_put(
        &self,
        message_id: i32,
        body: models::Message,
        context: &C) -> Result<MessagePutResponse, ApiError>;

    async fn auth_delete(
        &self,
        email: String,
        context: &C) -> Result<AuthDeleteResponse, ApiError>;

    async fn auth_get(
        &self,
        context: &C) -> Result<AuthGetResponse, ApiError>;

    async fn auth_get(
        &self,
        email: String,
        context: &C) -> Result<AuthGet_0Response, ApiError>;

    async fn auth_post(
        &self,
        body: models::User,
        context: &C) -> Result<AuthPostResponse, ApiError>;

    async fn auth_put(
        &self,
        email: String,
        body: models::User,
        context: &C) -> Result<AuthPutResponse, ApiError>;

    async fn register_get(
        &self,
        context: &C) -> Result<RegisterGetResponse, ApiError>;

    async fn register_post(
        &self,
        body: models::User,
        context: &C) -> Result<RegisterPostResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn message_get(
        &self,
        ) -> Result<MessageGetResponse, ApiError>;

    async fn message_post(
        &self,
        body: models::Message,
        ) -> Result<MessagePostResponse, ApiError>;

    async fn message_put(
        &self,
        message_id: i32,
        body: models::Message,
        ) -> Result<MessagePutResponse, ApiError>;

    async fn auth_delete(
        &self,
        email: String,
        ) -> Result<AuthDeleteResponse, ApiError>;

    async fn auth_get(
        &self,
        ) -> Result<AuthGetResponse, ApiError>;

    async fn auth_get(
        &self,
        email: String,
        ) -> Result<AuthGet_0Response, ApiError>;

    async fn auth_post(
        &self,
        body: models::User,
        ) -> Result<AuthPostResponse, ApiError>;

    async fn auth_put(
        &self,
        email: String,
        body: models::User,
        ) -> Result<AuthPutResponse, ApiError>;

    async fn register_get(
        &self,
        ) -> Result<RegisterGetResponse, ApiError>;

    async fn register_post(
        &self,
        body: models::User,
        ) -> Result<RegisterPostResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn message_get(
        &self,
        ) -> Result<MessageGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().message_get(&context).await
    }

    async fn message_post(
        &self,
        body: models::Message,
        ) -> Result<MessagePostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().message_post(body, &context).await
    }

    async fn message_put(
        &self,
        message_id: i32,
        body: models::Message,
        ) -> Result<MessagePutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().message_put(message_id, body, &context).await
    }

    async fn auth_delete(
        &self,
        email: String,
        ) -> Result<AuthDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().auth_delete(email, &context).await
    }

    async fn auth_get(
        &self,
        ) -> Result<AuthGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().auth_get(&context).await
    }

    async fn auth_get(
        &self,
        email: String,
        ) -> Result<AuthGet_0Response, ApiError>
    {
        let context = self.context().clone();
        self.api().auth_get(email, &context).await
    }

    async fn auth_post(
        &self,
        body: models::User,
        ) -> Result<AuthPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().auth_post(body, &context).await
    }

    async fn auth_put(
        &self,
        email: String,
        body: models::User,
        ) -> Result<AuthPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().auth_put(email, body, &context).await
    }

    async fn register_get(
        &self,
        ) -> Result<RegisterGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().register_get(&context).await
    }

    async fn register_post(
        &self,
        body: models::User,
        ) -> Result<RegisterPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().register_post(body, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
