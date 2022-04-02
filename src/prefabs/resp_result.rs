use std::{marker::PhantomData, task::Poll};

use futures::{pin_mut, Future};
use resp_result::{Nil, RespError, RespResult};

use crate::Treater;

pub struct ToRespResult<T: Treater + 'static>(PhantomData<T>);

impl<T: Treater + 'static> Treater for ToRespResult<T>
where
    T::Err: RespError + 'static,
    T::Fut: 'static,
{
    type Fut = ToRespResultFut<T>;

    type Resp = T::Resp;

    type Err = RespResult<Nil, T::Err>;

    fn proc(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Fut {
        let task = T::proc(req, payload);
        ToRespResultFut { inner_fut: task }
    }
}

#[pin_project::pin_project]
pub struct ToRespResultFut<T: Treater + 'static> {
    #[pin]
    inner_fut: T::Fut,
}

impl<T: Treater> Future for ToRespResultFut<T>
where
    T::Err: resp_result::RespError,
{
    type Output = Result<T::Resp, RespResult<Nil, T::Err>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let task = this.inner_fut;

        pin_mut!(task);

        match task.poll(cx) {
            Poll::Ready(resp) => Poll::Ready(match resp {
                Ok(data) => Ok(data),
                Err(err) => Err(RespResult::Err(err)),
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}
