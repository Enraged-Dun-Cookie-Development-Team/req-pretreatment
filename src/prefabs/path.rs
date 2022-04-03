use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use actix_web::FromRequest;

use crate::Treater;

use super::RawError;

pub struct PathValue<T: for<'de> serde::Deserialize<'de>>(PhantomData<T>);

impl<T> Treater for PathValue<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    type Fut = futures_util::future::Ready<Result<Self::Resp, Self::Err>>;

    type Resp = T;

    type Err = PathError;

    fn proc(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Fut {
        let task = actix_web::web::Path::<T>::from_request(req, payload)
            .into_inner()
            .map(actix_web::web::Path::into_inner)
            .map_err(|err| PathError(RawError::new(err)));
        futures_util::future::ready(task)
    }
}

pub struct PathError(RawError<actix_web::error::PathError>);

impl Debug for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for PathError{}
