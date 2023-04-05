use crate::{*};

#[derive(Default, Clone)]
pub struct Bg {
	pub col: u32
}

#[function_component]
pub fn bg(p: Bg, children: Elements)->Elements {
	let instance_ref=use_context::<AppContext>();
	let instance=instance_ref.borrow();

	instance.rect_renderer.draw(&instance.rect,p.col);

	children
}
