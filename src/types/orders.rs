use crate::types::Order;

#[derive(Debug, Deserialize)]
#[repr(C)]
pub struct Orders {
  /// ask side array of array entries(<price>, <volume>, <timestamp>)
  pub asks: Vec<Order>,
  /// bid side array of array entries(<price>, <volume>, <timestamp>)
  pub bids: Vec<Order>,
}
