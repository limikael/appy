#[path = "utils/export.rs"]
mod export;

pub use appy_macros::*;

//export!(render_env, "appy/render_env.rs");
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
export!(log, "utils/log.rs");
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

pub use Dim::{Pc, Px, Dp};

#[cfg(test)]
mod tests;
