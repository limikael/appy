use crate::core::app_context::AppContext;
use crate::core::element::{Element, Elements, ElementWrap};
use crate::core::hooks::use_context;
use appy::component;

/// Draws a single colored rectangle, filling the current [blk](crate::components::blk::blk()).
///
/// It is intented to be used inside [apx], e.g.:
///
/// ```rust
/// apx!{
///   <bg col=0x112233 />
///	}
///
#[component]
pub struct Bg {
	col: u32
}

impl Element for Bg {
    fn render(self: ElementWrap<Self>) -> Elements {
		let instance_ref=use_context::<AppContext>();
		let instance=instance_ref.borrow();

		instance.rect_renderer.draw(&instance.rect,self.col);
    	vec![]
    }
}
