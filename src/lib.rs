#[macro_use]
extern crate log;

pub mod horust;
pub use crate::horust::{get_sample_service, Horust, HorustError};
