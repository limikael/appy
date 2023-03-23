use crate::{*};
use std::rc::Rc;
use sdl2::event::Event;
use std::ops::Deref;

#[derive(Clone)]
pub struct HoverStateRef {
	pub current: Rc<HoverState>,
	pub set_current: Rc<dyn Fn(HoverState)>
}

impl Deref for HoverStateRef {
	type Target=HoverState;

	fn deref(&self)->&Self::Target {
		&*self.current
	}
}

impl HoverStateRef {
	pub fn new()->Self {
		let (current,set_current)=use_state(||HoverState::Normal);
		Self {
			current,
			set_current
		}
	}
}

impl Default for HoverStateRef {

	// this is really bad, causes use_state to be called in parent
	fn default()->Self {
		Self::new()
	}
}

pub fn use_hover_state_ref()->HoverStateRef {
	HoverStateRef::new()
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum HoverState {
	#[default]
	Normal,
	Hover,
	Active
}

#[derive(Clone, Default)]
pub struct Interaction {
	pub on_mouse_down: Cb,
	pub on_mouse_up: Cb,
	pub on_mouse_over: Cb,
	pub on_mouse_out: Cb,
	pub on_click: Cb,
	pub hover_state_ref: HoverStateRef
}

#[function_component]
pub fn interaction(p: Interaction, children:Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let rect={
		let instance=instance_ref.borrow();
		instance.rect.clone()
	};

	use_gl_window_event(Rc::new(with_clone!([],move|e|{
		match *e {
			Event::MouseButtonDown {x,y,..}=>{
				if rect.contains(x,y) {
					(*p.hover_state_ref.set_current)(HoverState::Active);
					(p.on_mouse_down)();
				}
			},
			Event::MouseButtonUp {x,y,..}=>{
				if rect.contains(x,y) {
					(p.on_mouse_up)();
					if *p.hover_state_ref==HoverState::Active {
						(p.on_click)();
					}

					(*p.hover_state_ref.set_current)(HoverState::Hover);
				}
			},
			Event::MouseMotion {x,y,..}=>{
				if rect.contains(x,y) && *p.hover_state_ref==HoverState::Normal {
					(*p.hover_state_ref.set_current)(HoverState::Hover);
					(p.on_mouse_over)();
				}

				else if !rect.contains(x,y) && *p.hover_state_ref!=HoverState::Normal {
					(*p.hover_state_ref.set_current)(HoverState::Normal);
					(p.on_mouse_out)();
				}
			},
			_ => {},
		}
	})));


	children
}

