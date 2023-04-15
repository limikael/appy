/// Reference to a callback.
pub mod cb;

/// OpenGL utilities.
pub mod glutil;

/// OpenGL utilities.
pub mod rect;

/// Render rectangles.
pub mod rect_renderer;

/// Render text.
pub mod text_renderer;

/// Abstraction for an OpenGL shader program. 
pub mod shader_program;

/// Abstraction for an OpenGL array buffer.
pub mod array_buffer;

/// Trigger that can be notified remotely.
pub mod trigger;

/// Clone variables for closure.
pub mod with_clone;

#[cfg(all(target_os="android",feature="sdl"))]
pub mod android_log_thread;
