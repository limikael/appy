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
//! use appy::*;
//!
//! #[main_window]
//! pub fn app()->Elements {
//!    apx!{
//!        <bg col=0x800000/>
//!        <text text="Hello World".to_string() align=Align::Center/>
//!    }
//! }
//! ```
//!

pub use appy_macros::*;

#[doc(hidden)]
pub mod core;

/// Graphical UI components.
pub mod components;

#[doc(hidden)]
pub mod sys;

/// Utilities.
pub mod utils;

/// Types used by hooks and components.
pub mod types;

/// Hooks that let you hook into the system for storing state and more.
pub mod hooks;

#[cfg(test)]
mod tests;

extern crate self as appy;