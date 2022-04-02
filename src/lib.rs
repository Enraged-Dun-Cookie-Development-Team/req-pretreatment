pub mod prefabs;
mod pretreat;
mod pretreat_fut;

pub use pretreat::{Pretreatment, Treater};
pub use pretreat_fut::PretreatFuture;

pub type PreRResult<T, E> = Pretreatment<prefabs::MapError<prefabs::ToRespResult<T>, E>>;
