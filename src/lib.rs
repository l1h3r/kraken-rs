#![feature(box_syntax, slice_concat_trait)]

#[macro_use]
extern crate serde;

pub mod client;
pub mod consts;
mod de;
pub mod types;
