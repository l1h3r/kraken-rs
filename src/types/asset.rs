#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Asset {
  /// scaling decimal places for record keeping
  pub decimals: u32,
  /// scaling decimal places for output display
  pub display_decimals: u32,
  /// asset class
  pub aclass: String,
  /// alternate name
  pub altname: String,
}
