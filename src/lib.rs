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

/// Core functions.
pub mod core;

/// Graphical UI components.
pub mod components;

/// Abstraction so that Appy can use different libraries for window and context management.
pub mod sys;

/// Utilities.
pub mod utils;

/*#[cfg(all(target_os="android",feature="glutin"))]
pub use winit::platform::android::activity::AndroidApp;*/

#[cfg(test)]
mod tests;

extern crate self as appy;