use std::marker::PhantomData;

use actix_web::error::QueryPayloadError;

use crate::Treater;

pub struct QueryArgs<T: for<'de> serde::Deserialize<'de>>(PhantomData<T>);

impl<T> Treater for QueryArgs<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    type Fut = futures_util::future::Ready<Result<Self::Resp, Self::Err>>;

    type Resp = T;

    type Err = QueryPayloadError;

    fn proc(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Fut {
        let query = req.query_string();
        let q =
            actix_web::web::Query::<T>::from_query(query).map(actix_web::web::Query::into_inner);

        futures_util::future::ready(q)
    }
}
