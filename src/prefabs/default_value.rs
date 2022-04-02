use std::{convert::Infallible, marker::PhantomData};

use futures::future::ok;

use crate::Treater;

pub struct DefaultValue<T: Default>(PhantomData<T>);

impl<T: Default> Treater for DefaultValue<T> {
    type Fut = futures_util::future::Ready<Result<Self::Resp, Self::Err>>;

    type Resp = T;

    type Err = Infallible;

    fn proc(_: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Fut {
        ok(T::default())
    }
}
