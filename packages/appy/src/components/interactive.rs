use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

#[derive(Clone, Default)]
pub struct Interactive {
	pub on_mouse_down: Cb
}

#[function_component]
pub fn interactive(p: Interactive, children:Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let rect={
		let instance=instance_ref.borrow();
		instance.rect.clone()
	};

	use_gl_window_event(Rc::new(with_clone!([],move|e|{
		match *e {
			Event::MouseButtonDown {x, y, ..} => {
				if rect.contains(x,y) {
					(p.on_mouse_down)();
				}
			},
			_ => {},
		}
	})));


	children
}

