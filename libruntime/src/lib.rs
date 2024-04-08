#[macro_use]
extern crate lazy_static;

pub mod context;
pub mod create;
pub mod state;
mod error;
mod fork;
mod fs;
mod ipc;
pub mod log;
mod store;
mod namespace;
pub mod start;

lazy_static!{
    pub static ref OCI_VERSION: String = String::from("0.2.0");
}