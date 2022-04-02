use std::{marker::PhantomData, task::Poll};

use futures::{pin_mut, Future};

use crate::Treater;

pub struct MapError<T: Treater, E: From<T::Err>>(PhantomData<(T, E)>);

impl<T, E> Treater for MapError<T, E>
where
    T: Treater,
    E: From<T::Err>,
    T: 'static,
{
    type Fut = MapErrFut<T, E>;

    type Resp = T::Resp;

    type Err = E;

    fn proc(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Fut {
        let task = T::proc(req, payload);
        MapErrFut {
            task,
            _err: Default::default(),
        }
    }
}

#[pin_project::pin_project]
pub struct MapErrFut<T: Treater + 'static, E: From<T::Err>> {
    #[pin]
    task: T::Fut,
    _err: PhantomData<E>,
}

impl<T: Treater + 'static, E: From<T::Err>> Future for MapErrFut<T, E> {
    type Output = Result<T::Resp, E>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let task = this.task;

        pin_mut!(task);

        match task.poll(cx) {
            Poll::Ready(r) => match r {
                Ok(data) => Poll::Ready(Ok(data)),
                Err(err) => Poll::Ready(Err(E::from(err))),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
