#[path="utils/export.rs"]
mod export;

pub use appy_macros::{*};

export!(render_env,"appy/render_env.rs");
export!(hooks,"appy/hooks.rs");
export!(appy,"appy/appy.rs");
export!(element,"appy/element.rs");

export!(window,"components/window.rs");
export!(bg,"components/bg.rs");
export!(blk,"components/blk.rs");
export!(text,"components/text.rs");
export!(interaction,"components/interaction.rs");

export!(glutil,"utils/glutil.rs");
export!(trigger,"utils/trigger.rs");
export!(log,"utils/log.rs");
export!(cb,"utils/cb.rs");
export!(with_clone,"utils/with_clone.rs");
export!(rect,"utils/rect.rs");
export!(shader_program,"utils/shader_program.rs");
export!(array_buffer,"utils/array_buffer.rs");
export!(rect_renderer,"utils/rect_renderer.rs");
export!(text_renderer,"utils/text_renderer.rs");

pub use Dim::{Px,Pc};

#[cfg(test)]
mod tests;