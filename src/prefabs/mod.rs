mod query;
mod default_value;
mod error_mapping;
mod json;
mod path;
mod resp_result;

use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::Deref;

use actix_web::ResponseError;

pub use self::default_value::DefaultValue;
pub use self::error_mapping::{MapErrFut, MapError};
pub use self::json::{JsonError, JsonPayload, JsonTreaterFut};
pub use self::path::{PathError, PathValue};
pub use self::query::QueryArgs;
pub use self::resp_result::{ToRespResult, ToRespResultFut};

pub type Non = DefaultValue<()>;

struct RawError<T: ResponseError + 'static> {
    raw: actix_web::Error,
    _phantom: PhantomData<T>,
}

impl<T: ResponseError + 'static> Deref for RawError<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.raw.as_error::<T>().unwrap()
    }
}

impl<T: ResponseError + 'static> RawError<T> {
    fn new(raw: actix_web::Error) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

impl<T: ResponseError + 'static> Display for RawError<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.deref(), f)
    }
}
impl<T: ResponseError + 'static> Debug for RawError<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}
