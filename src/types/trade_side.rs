#[derive(Clone, Copy, Debug, Deserialize)]
#[repr(u8)]
pub enum TradeSide {
  #[serde(rename = "b")]
  Buy,
  #[serde(rename = "s")]
  Sell,
}
