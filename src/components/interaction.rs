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

pub fn use_hover_state_ref()->HoverStateRef {
	HoverStateRef::new()
}

#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum HoverState {
	#[default]
	Normal,
	Hover,
	Active
}

#[derive(Clone, Default)]
pub struct Interaction {
	/*pub on_mouse_down: Cb,
	pub on_mouse_up: Cb,
	pub on_mouse_over: Cb,
	pub on_mouse_out: Cb,*/
	pub on_click: Cb,
	pub hover_state_ref: Option<HoverStateRef>
}

const SDL_TOUCH_MOUSEID:u32 = u32::MAX;

#[function_component]
pub fn interaction(p: Interaction, children:Elements)->Elements {
	let (h_state,set_h_state)=use_state(||HoverState::Normal);
	let instance_ref=use_context::<GlWindowInstance>();
	let rect={
		let instance=instance_ref.borrow();
		instance.rect.clone()
	};

	if p.hover_state_ref.is_some() &&
			p.hover_state_ref.as_ref().unwrap().current!=h_state {
		panic!("they are different!!!");
	}

	let update_h_state=rc_with_clone!([],move|new_state|{
		set_h_state(new_state);
		if p.hover_state_ref.is_some() {
			(p.hover_state_ref.as_ref().unwrap().set_current)(new_state);
		}
	});

	use_gl_window_event(rc_with_clone!([],move|e|{

		match *e {
			Event::MouseButtonDown {x,y,..}=>{
				if rect.contains(x,y) {
					update_h_state(HoverState::Active);
				}
			},
			Event::MouseButtonUp {x,y,which,..}=>{
				if rect.contains(x,y) {
					if *h_state==HoverState::Active {
						(p.on_click)();
					}

					if which==SDL_TOUCH_MOUSEID {
						update_h_state(HoverState::Normal);
					}

					else {
						update_h_state(HoverState::Hover);
					}
				}
			},
			Event::MouseMotion {x,y,which,..}=>{
				if rect.contains(x,y) &&
						*h_state==HoverState::Normal &&
						which!=SDL_TOUCH_MOUSEID {
					update_h_state(HoverState::Hover);
				}

				else if !rect.contains(x,y) && *h_state!=HoverState::Normal {
					update_h_state(HoverState::Normal);
				}
			},
			_ => {},
		}
	}));

	children
}

