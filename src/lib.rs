//! # Appy - Declarative UI Framework for Native Application
//!
//! Appy is a Rust framework for building native UI applications with a declarative syntax.
//!
//! - Inspired by React, it uses familiar concepts like function components and hooks.
//! - Draws elements directly using hardware acceleration and OpenGL. There is no
//!   DOM involved.
//! - Supports multiple rendering backeds, such as Glutin and SDL, which ensures
//!   cross-platform operation with a single application code base.
//!
//! ## Example
//! 
//! ```rust
//! use appy::{*, types::*, components::*};
//!
//! #[main_window]
//! pub fn app()->Elements {
//!    apx!{
//!        <bg color=0x800000/>
//!        <text text="Hello World" align=Align::Center/>
//!    }
//! }
//! ```
//!

pub use appy_macros::*;
pub use glapp;
pub use glapp::gl;

#[doc(hidden)]
pub mod core;

/// Graphical UI components.
pub mod components;

/// Utilities.
pub mod utils;

/// Types used by hooks and components.
pub mod types;

/// Hooks that let you hook into the system for storing state and more.
pub mod hooks;

#[cfg(test)]
mod tests;

extern crate self as appy;