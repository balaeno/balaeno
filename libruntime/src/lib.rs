#[macro_use]
extern crate lazy_static;

pub mod context;
pub mod create;
pub mod state;
mod error;
mod fs;
pub mod log;
mod entry;
mod store;

lazy_static!{
    pub static ref OCI_VERSION: String = String::from("0.2.0");
}