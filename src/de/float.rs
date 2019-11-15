use serde::de::Error;
use serde::de::Visitor;
use serde::Deserializer;
use std::fmt::Formatter;
use std::fmt::Result as FResult;

struct F64;

impl<'de> Visitor<'de> for F64 {
  type Value = f64;

  fn expecting(&self, f: &mut Formatter) -> FResult {
    f.write_str("f64 as a number or string")
  }

  fn visit_f64<E: Error>(self, id: f64) -> Result<Self::Value, E> {
    Ok(id)
  }

  fn visit_str<E: Error>(self, string: &str) -> Result<Self::Value, E> {
    string.parse().map_err(Error::custom)
  }
}

pub fn deserialize_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
  deserializer.deserialize_any(F64)
}
