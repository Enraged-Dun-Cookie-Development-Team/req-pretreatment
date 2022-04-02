use std::{future::Future, ops::Deref};

use crate::pretreat_fut;

/// 预处理器，用于分级预处理数据
pub trait Treater {
    /// 异步进行数据预处理的Future
    type Fut: Future<Output = Result<Self::Resp, Self::Err>>;
    /// 预处理器的返回结果
    type Resp;
    /// 预处理器出现异常时的返回类型信息
    type Err;
    /// 进行预处理操作的函数
    fn proc(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Fut;
}

pub struct Pretreatment<T: Treater>(pub T::Resp);

impl<T: Treater> Pretreatment<T> {
    /// 拆箱操作
    /// 将内部结果提取出来
    pub fn unwrap(self) -> T::Resp {
        self.0
    }
    /// 行为同 [Self::unwrap]
    pub fn into_inner(self) -> T::Resp {
        self.unwrap()
    }
}

impl<T: Treater> Deref for Pretreatment<T> {
    type Target = T::Resp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r, T: Treater> actix_web::FromRequest for Pretreatment<T>
where
    T: 'static,
    T::Err: 'static,
    T::Err: actix_web::ResponseError,
{
    type Error = T::Err;

    type Future = pretreat_fut::PretreatFuture<T>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        pretreat_fut::PretreatFuture::new(req, payload)
    }
}

