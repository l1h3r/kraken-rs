use crate::de::deserialize_f64;

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Level {
  #[serde(deserialize_with = "deserialize_f64")]
  pub price: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub whole_lot_volume: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub lot_volume: f64,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct CloseLevel {
  #[serde(deserialize_with = "deserialize_f64")]
  pub price: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub volume: f64,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct TimeLevel {
  #[serde(deserialize_with = "deserialize_f64")]
  pub today: f64,
  #[serde(deserialize_with = "deserialize_f64")]
  pub last24: f64,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct TickerPair {
  /// ask array(<price>, <whole lot volume>, <lot volume>),
  pub a: Level,
  /// bid array(<price>, <whole lot volume>, <lot volume>),
  pub b: Level,
  /// last trade closed array(<price>, <lot volume>),
  pub c: CloseLevel,
  /// volume array(<today>, <last 24 hours>),
  pub v: TimeLevel,
  /// volume weighted average price array(<today>, <last 24 hours>),
  pub p: TimeLevel,
  /// number of trades array(<today>, <last 24 hours>),
  pub t: [u32; 2],
  /// low array(<today>, <last 24 hours>),
  pub l: TimeLevel,
  /// high array(<today>, <last 24 hours>),
  pub h: TimeLevel,
  /// today's opening price
  #[serde(deserialize_with = "deserialize_f64")]
  pub o: f64,
}
