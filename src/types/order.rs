use crate::de::deserialize_f64;

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct Order {
  #[serde(deserialize_with = "deserialize_f64")]
  pub price: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub volume: f64,
  pub timestamp: u64,
}
