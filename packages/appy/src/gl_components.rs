use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

#[derive(Clone)]
pub struct InteractiveProps {
	pub x: i32, pub y: i32, pub w: i32, pub h: i32,
	pub on_mouse_down: Rc<dyn Fn()>
}

#[function_component]
fn interactive(p: InteractiveProps, _children:Elements)->Elements {
	let s=p.clone();
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

#[derive(Clone)]
pub struct ButtonProps {
	pub x: i32, pub y: i32, pub w: i32, pub h: i32,
	pub on_click: Rc<dyn Fn()>
}

#[function_component]
pub fn button(props: ButtonProps, _children: Elements)->Elements {
	let s=props.clone();

	let on_mouse_down=Rc::new(move||{
		(s.on_click)();
	});

	apx!{
		<rect x="s.x" y="s.y" w="s.w" h="s.h"/>
		<interactive
				x="s.x" y="s.y" w="s.w" h="s.h"
				on_mouse_down="on_mouse_down.clone()"/>
	}
}

pub struct RectProps {
	pub x: i32, pub y: i32, pub w: i32, pub h: i32,
}

#[function_component]
pub fn rect(p: RectProps, children: Vec<Element>)->Vec<Element> {
	let instance=use_context::<GlWindowInstance>();
	instance.borrow().rect_renderer.draw(p.x,p.y,p.w,p.h);
	apx!{}
}