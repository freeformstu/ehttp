//! Example application using [`eframe`].

mod app;

pub use app::DemoApp;

// ----------------------------------------------------------------------------

#[cfg(target_family = "wasm")]
mod web;

#[cfg(target_family = "wasm")]
pub use web::*;
