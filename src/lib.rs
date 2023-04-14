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
#[path = "utils/export.rs"]
mod export;

pub use appy_macros::*;

export!(hooks, "appy/hooks.rs");
export!(appy, "appy/appy.rs");
export!(element, "appy/element.rs");
export!(app_context, "appy/app_context.rs");
export!(component, "appy/component.rs");

export!(bg, "components/bg.rs");
export!(blk, "components/blk.rs");
export!(text, "components/text.rs");
export!(interaction, "components/interaction.rs");
export!(grid, "components/grid.rs");

export!(glutil, "utils/glutil.rs");
export!(trigger, "utils/trigger.rs");
export!(cb, "utils/cb.rs");
export!(with_clone, "utils/with_clone.rs");
export!(rect, "utils/rect.rs");
export!(shader_program, "utils/shader_program.rs");
export!(array_buffer, "utils/array_buffer.rs");
export!(rect_renderer, "utils/rect_renderer.rs");
export!(text_renderer, "utils/text_renderer.rs");

export!(app_window, "sys/app_window.rs");

#[cfg(feature="glutin")]
export!(app_window_glutin, "sys/app_window_glutin.rs");

#[cfg(feature="sdl")]
export!(app_window_sdl, "sys/app_window_sdl.rs");

#[cfg(all(target_os="android",feature="glutin"))]
pub use winit::platform::android::activity::AndroidApp;

#[cfg(all(target_os="android",feature="sdl"))]
export!(android_log_thread,"sys/android_log_thread.rs");

pub use Dim::{Pc, Px, Dp};

#[cfg(test)]
mod tests;
