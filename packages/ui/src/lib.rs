//! This crate contains all shared UI for the workspace.

pub mod components;
pub use components::*;

mod capture;
pub use capture::Capture;

mod markdown;
pub use markdown::render_markdown;
