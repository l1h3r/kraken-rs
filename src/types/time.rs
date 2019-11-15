#[derive(Debug, Deserialize)]
#[repr(C)]
#[serde(deny_unknown_fields)]
pub struct Time {
  /// server time as unix timestamp
  pub unixtime: u64,
  /// server time as RFC 1123 time format
  pub rfc1123: String,
}
