use crate::de::deserialize_f64;

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Spread {
  pub time: u64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub bid: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub ask: f64,
}
