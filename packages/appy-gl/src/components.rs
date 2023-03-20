use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

/*#[derive(Clone)]
pub struct InteractiveProps {
	pub x: i32, pub y: i32, pub w: i32, pub h: i32,
	pub on_mouse_down: Rc<dyn Fn()>
}

#[function_component]
fn interactive(p: InteractiveProps, _children:Elements)->Elements {
	use_gl_window_event(Rc::new(with_clone!([p],move|e|{
		match *e {
			Event::MouseButtonDown {x, y, ..} => {
				if x>=p.x && y>=p.y && x <p.x+p.w && y <p.y+p.h {
					(p.on_mouse_down)();
				}
			},
			_ => {},
		}
	})));

	apx!{}
}

#[derive(Clone)]
pub struct ButtonProps {
	pub x: i32, pub y: i32, pub w: i32, pub h: i32,
	pub on_click: Rc<dyn Fn()>
}

#[function_component]
pub fn button(p: ButtonProps, _children: Elements)->Elements {
	let on_mouse_down=Rc::new(with_clone!([p],move||{
		(p.on_click)();
	}));

	apx!{
		<rect x=p.x y=p.y w=p.w h=p.h/>
		<interactive
				x=p.x y=p.y w=p.w h=p.h
				on_mouse_down=on_mouse_down/>
	}
}
*/

pub struct RectProps {
	pub col: u32
}

#[function_component]
pub fn rect(p: RectProps, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let instance=instance_ref.borrow();

	instance.rect_renderer.draw(&instance.rect,p.col);

	children
}

pub struct AbsProps {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}

#[function_component]
pub fn abs(p: AbsProps, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let mut instance=instance_ref.borrow_mut();

	instance.rect=instance.rect.abs(p.x,p.y,p.w,p.h);

	children
}