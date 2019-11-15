use crate::de::deserialize_f64;
use crate::types::TradeSide;
use crate::types::TradeType;

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Trade {
  #[serde(deserialize_with = "deserialize_f64")]
  pub price: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub volume: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub time: f64,
  pub side: TradeSide,
  pub type_: TradeType,
  pub miscellaneous: String,
}
