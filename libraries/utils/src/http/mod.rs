//! Some utilities related to HTTP

mod client;
pub mod error;
mod params;
pub mod rest;

pub use client::HttpClient;
pub use error::Error;
pub use params::AddToParams;
