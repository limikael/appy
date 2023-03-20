use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

#[derive(Clone)]
pub struct Interactive {
	pub on_mouse_down: Rc<dyn Fn()>
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

pub struct Abs {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}

#[function_component]
pub fn abs(p: Abs, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let mut instance=instance_ref.borrow_mut();

	let old_rect=instance.rect.clone();
	instance.rect=instance.rect.abs(p.x,p.y,p.w,p.h);

	use_post_render(Rc::new(with_clone!([instance_ref],move||{
		let mut instance=instance_ref.borrow_mut();
		instance.rect=old_rect.clone();
	})));

	children
}

pub struct Rel {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}

#[function_component]
pub fn rel(p: Rel, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let instance=instance_ref.borrow_mut();
	let rect=instance.rect.clone();

	let abs_props=Abs{
		x: p.x*rect.w/100,
		y: p.y*rect.h/100,
		w: p.w*rect.w/100,
		h: p.h*rect.h/100,
	};

	vec![
		Element::create(abs,abs_props,children)
	]

/*	apx!{
		<abs>
			{children}
		</abs>
	}*/
}
