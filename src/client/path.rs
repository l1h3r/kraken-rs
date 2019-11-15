#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Path(String);

impl Path {
  #[inline(always)]
  pub fn new(path: &str) -> Self {
    Self(path.to_string())
  }

  #[inline(always)]
  pub fn as_str(&self) -> &str {
    // format!("{}?{}", self.path, self.segments.into_iter().map(|segment| segment.join("=")).join("&"))
    &self.0
  }

  pub fn add<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
    if self.0.contains("?") {
      self.0.push('&');
    } else {
      self.0.push('?');
    }

    self.0.push_str(key.as_ref());
    self.0.push('=');
    self.0.push_str(value.as_ref());
  }

  pub fn add_slice<K: AsRef<str>>(&mut self, key: K, slice: &[&str]) {
    if !slice.is_empty() {
      self.add(key, slice.join(","));
    }
  }
}
