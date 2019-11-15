#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Fee {
  pub volume: f64,
  pub percent: f64,
}

#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct AssetPair {
  /// alternate pair name
  pub altname: String,
  /// WebSocket pair name (if available)
  pub wsname: Option<String>,
  /// asset class of base component
  pub aclass_base: String,
  /// asset id of base component
  pub base: String,
  /// asset class of quote component
  pub aclass_quote: String,
  /// asset id of quote component
  pub quote: String,
  /// volume lot size
  pub lot: String,
  /// scaling decimal places for pair
  pub pair_decimals: u32,
  /// scaling decimal places for volume
  pub lot_decimals: u32,
  /// amount to multiply lot volume by to get currency volume
  pub lot_multiplier: u32,
  /// leverage_buy = array of leverage amounts available when buying
  pub leverage_buy: Vec<f64>,
  /// leverage_sell = array of leverage amounts available when selling
  pub leverage_sell: Vec<f64>,
  /// fee schedule array in [volume, percent fee] tuples
  pub fees: Vec<Fee>,
  /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
  pub fees_maker: Option<Vec<Fee>>,
  /// volume discount currency
  pub fee_volume_currency: String,
  /// margin call level
  pub margin_call: u32,
  /// stop-out/liquidation margin level
  pub margin_stop: u32,
}
