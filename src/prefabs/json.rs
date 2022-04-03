use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    pin::Pin,
    task::Poll,
};

use actix_web::{error::JsonPayloadError, web, FromRequest};
use futures::{ready, Future};
use pin_project::pin_project;

use serde::Deserialize;

use crate::Treater;

use super::RawError;

pub struct JsonPayload<T: for<'de> Deserialize<'de>>(PhantomData<T>);

impl<T> Treater for JsonPayload<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Fut = JsonTreaterFut<T>;

    type Resp = T;

    type Err = JsonError;

    fn proc(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Fut {
        let task = web::Json::<T>::from_request(req, payload);
        JsonTreaterFut { task }
    }
}

pub struct JsonError(RawError<JsonPayloadError>);

impl Debug for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl std::error::Error for JsonError {}

#[pin_project]
pub struct JsonTreaterFut<T>
where
    T: for<'de> Deserialize<'de>,
{
    #[pin]
    task: <actix_web::web::Json<T> as FromRequest>::Future,
}

impl<T> Future for JsonTreaterFut<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Output = Result<T, JsonError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let result = ready!(this.task.poll(cx));

        Poll::Ready(match result {
            Ok(data) => Ok(data.into_inner()),
            Err(err) => Err(JsonError(RawError::new(err))),
        })
    }
}
