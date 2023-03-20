use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;

#[derive(Clone)]
pub struct Interactive {
	pub on_mouse_down: Rc<dyn Fn()>
}

impl Default for Interactive {
	fn default()->Self {
		Self {
			on_mouse_down: Rc::new(||{})
		}
	}
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

#[derive(Default)]
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

#[derive(Default, Clone)]
pub enum Dim {
	#[default]
	None,
	Pc(f32),
	Px(f32)
}

impl Dim {
	pub fn is_some(&self)->bool {
		match *self {
			Dim::None=>false,
			_=>true
		}
	}

	pub fn to_px(&self, max: f32)->f32 {
		match *self {
			Dim::Px(v)=>v,
			Dim::Pc(v)=>max*v/100.0,
			Dim::None=>panic!("can't get px from none")
		}
	}

	pub fn compute_span(max:f32, start:Dim, size:Dim, end:Dim)->(f32,f32) {
		let c=
			if start.is_some() {1<<2} else {0}+
			if size.is_some() {1<<1} else {0}+
			if end.is_some() {1<<0} else {0};

		match c {
			0b000=>(0.0,max),
			0b001=>(0.0,max-end.to_px(max)),
			0b010=>((max-size.to_px(max))/2.0,size.to_px(max)),
			0b011=>(max-size.to_px(max)-end.to_px(max),size.to_px(max)),
			0b100=>(start.to_px(max),max-start.to_px(max)),
			0b101=>(start.to_px(max),max-start.to_px(max)-end.to_px(max)),
			0b110=>(start.to_px(max),size.to_px(max)),
			0b111=>panic!("over constrained"),
			_=>panic!("bad constraints")
		}
	}
}

impl Dim {
}

#[derive(Default)]
pub struct Blk {
	pub left: Dim,
	pub top: Dim,
	pub width: Dim,
	pub height: Dim,
	pub bottom: Dim,
	pub right: Dim
}

#[function_component]
pub fn blk(p: Blk, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let mut instance=instance_ref.borrow_mut();

	let old_rect=instance.rect.clone();
	let h=Dim::compute_span(old_rect.w as f32,p.left,p.width,p.right);
	let v=Dim::compute_span(old_rect.h as f32,p.top,p.height,p.bottom);

	instance.rect=instance.rect.abs(
		h.0 as i32, v.0 as i32,
		h.1 as i32, v.1 as i32
	);

	use_post_render(Rc::new(with_clone!([instance_ref],move||{
		let mut instance=instance_ref.borrow_mut();
		instance.rect=old_rect.clone();
	})));

	children
}

/*#[function_component]
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
}*/
