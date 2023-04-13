use crate::core::app_context::AppContext;
use crate::core::element::Elements;
use crate::core::hooks::use_context;
use crate::{*};

/// Props for the [bg](bg()) function component.
#[derive(Default, Clone)]
pub struct Bg {
	pub col: u32
}

/// Draws a single colored rectangle, filling the current [blk](blk()).
///
/// It is intented to be used inside [apx], e.g.:
///
/// ```rust
/// apx!{
///   <bg col=0x112233 />
///	}
///
#[function_component]
pub fn bg(p: Bg, children: Elements)->Elements {
	let instance_ref=use_context::<AppContext>();
	let instance=instance_ref.borrow();

	instance.rect_renderer.draw(&instance.rect,p.col);

	children
}
