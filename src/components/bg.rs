use crate::types::{AppContext, Element, Elements, ElementWrap};
use crate::hooks::use_context;
use appy::{derive_component, SnakeFactory, ComponentBuilder};

/// Draws a single colored rectangle, filling the current [Blk](crate::components::Blk).
///
/// It is intented to be used inside [crate::apx], e.g.:
///
/// ```rust
/// apx!{
///   <bg col=0x112233 />
///	}
///
#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Bg {
	col: u32
}

impl Element for Bg {
    fn render(self: ElementWrap<Self>) -> Elements {
		let app_context=use_context::<AppContext>();

		app_context.rect_renderer.borrow().draw(&app_context.rect,self.col);
    	vec![]
    }
}
