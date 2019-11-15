use std::collections::HashMap;
use std::error::Error;

pub type BoxError = Box<dyn Error>;
pub type Map<T> = HashMap<String, T>;

mod asset;
mod asset_pair;
mod candle;
mod interval;
mod order;
mod orders;
mod response;
mod spread;
mod ticker_pair;
mod time;
mod trade;
mod trade_side;
mod trade_type;

pub use self::asset::*;
pub use self::asset_pair::*;
pub use self::candle::*;
pub use self::interval::*;
pub use self::order::*;
pub use self::orders::*;
pub use self::response::*;
pub use self::spread::*;
pub use self::ticker_pair::*;
pub use self::time::*;
pub use self::trade::*;
pub use self::trade_side::*;
pub use self::trade_type::*;
