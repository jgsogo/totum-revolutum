//! Some utilities related to HTTP

// pub mod http;
mod client;
mod params;
pub mod rest;

pub use client::HttpClient;
pub use params::AddToParams;
