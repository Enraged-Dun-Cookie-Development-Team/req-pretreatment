mod prefabs;
mod pretreat;
mod pretreat_fut;

use prefabs::ToRespResult;
pub use pretreat::Pretreatment;
pub use pretreat::Treater;
pub use pretreat_fut::PretreatFuture;


pub type PreRResult<T> =Pretreatment<ToRespResult<T>>;