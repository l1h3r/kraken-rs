use crate::types::Candle;
use crate::types::Map;
use crate::types::Spread;
use crate::types::Trade;

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Response<T> {
  pub error: Vec<String>,
  pub result: T,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct OHLCResponse {
  #[serde(flatten)]
  pub data: Map<Vec<Candle>>,
  pub last: u64,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct TradeResponse {
  #[serde(flatten)]
  pub data: Map<Vec<Trade>>,
  pub last: String, // TODO: u64
}

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct SpreadResponse {
  #[serde(flatten)]
  pub data: Map<Vec<Spread>>,
  pub last: u64,
}
