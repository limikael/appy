use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

#[component]
pub struct Interactive {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
	pub on_mouse_down: Rc<dyn Fn()>
}

impl Component for Interactive {
	fn render(&self)->ComponentFragment {
		let s=(*self).clone();
		use_gl_window_event(Rc::new(move|e|{
			match *e {
				Event::MouseButtonDown {x, y, ..} => {
					if x>=s.x && y>=s.y && x <s.x+s.w && y <s.y+s.h {
						(s.on_mouse_down)();
					}
				},
				_ => {},
			}
		}));

		apx!{}
	}
}

#[component]
pub struct Button {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
	pub on_click: Rc<dyn Fn()>
}

impl Component for Button {
	fn render(&self)->ComponentFragment {
		let s=(*self).clone();

		let on_mouse_down=move||{
			(s.on_click)();
		};

		apx!{
			<Rect x="self.x" y="self.y" w="self.w" h="self.h"/>
			<Interactive
					x="self.x" y="self.y" w="self.w" h="self.h"
					on_mouse_down="Rc::new(on_mouse_down)"/>
		}
	}
}

#[component]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}

impl Component for Rect {
	fn render(&self)->ComponentFragment {
		let instance=use_context::<GlWindowInstance>();
		instance.borrow().rect_renderer.draw(self.x,self.y,self.w,self.h);
		apx!{}
	}
}
