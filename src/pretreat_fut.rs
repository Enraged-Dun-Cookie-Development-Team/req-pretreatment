use std::{future::Future, task::Poll};

use crate::pretreat::{Pretreatment, Treater};

#[pin_project::pin_project]
/// 构造预处理器Future 
pub struct PretreatFuture<T: Treater + 'static> {
    #[pin]
    fut: T::Fut,
}

impl<T: Treater + 'static> PretreatFuture<T> {
    pub(crate) fn new(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self {
        Self {
            fut: T::proc(req, payload),
        }
    }
}

impl<T> Future for PretreatFuture<T>
where
    T: Treater + 'static,
{
    type Output = Result<Pretreatment<T>, T::Err>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let task = this.fut;

        match task.poll(cx) {
            Poll::Ready(Ok(data)) => Poll::Ready(Ok(Pretreatment(data))),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}
