use crate::de::deserialize_f64;

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Candle {
  pub time: u64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub open: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub high: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub low: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub close: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub vwap: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub volume: f64,
  pub count: u64,
}
