#[derive(Clone, Copy, Debug, Deserialize)]
#[repr(u8)]
pub enum TradeType {
  #[serde(rename = "m")]
  Market,
  #[serde(rename = "l")]
  Limit,
}
