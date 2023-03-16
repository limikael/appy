use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use crate::{Component, ComponentFragment};

#[derive(PartialEq)]
pub enum IdleAction {
	None,
	Redraw,
	Quit
}

#[derive(Clone)]
pub enum SignalHandler {
	PostRender(Rc<dyn Fn()>),
	Idle(Rc<dyn Fn()->IdleAction>)
}

pub struct ComponentInstance {
	hook_data: Vec<Rc<dyn Any>>,
}

impl ComponentInstance {
	pub fn new()->Self {
		Self {
			hook_data: vec![],
		}
	}
}

thread_local! {
	static CURRENT_RENDER_ENV:RefCell<Option<Rc<RefCell<RenderEnv>>>>=RefCell::new(None);
}

pub struct RenderEnv {
	component_instance: Rc<RefCell<ComponentInstance>>,
	hook_index: usize,
	pub signal_handlers: Vec<SignalHandler>
}

impl RenderEnv {
	pub fn render(c: Rc<dyn Component>, ci: Rc<RefCell<ComponentInstance>>)->
			(ComponentFragment, Vec<SignalHandler>) {
		let re=RenderEnv{
			component_instance: ci,
			hook_index: 0,
			signal_handlers: vec![]
		};

		RenderEnv::set_current(Some(Rc::new(RefCell::new(re))));
		let child_fragment=c.render();
		let signal_handlers=RenderEnv::get_current().borrow().signal_handlers.clone();
		RenderEnv::set_current(None);

		(child_fragment, signal_handlers)
	}

	pub fn get_current()->Rc<RefCell<RenderEnv>> {
		CURRENT_RENDER_ENV.with(|instance| {
			instance.borrow().clone().unwrap().clone()
		})
	}

	fn set_current(c: Option<Rc<RefCell<RenderEnv>>>) {
		CURRENT_RENDER_ENV.with(|instance| {
			*instance.borrow_mut()=c;
		});
	}

	pub fn get_current_hook_data<F, T: 'static>(&mut self, ctor: F)->Rc<RefCell<T>>
			where F: Fn()->T {
		let mut ci=self.component_instance.borrow_mut();

		if self.hook_index>=ci.hook_data.len() {
			ci.hook_data.push(Rc::new(RefCell::new(ctor())));
		}

		let use_hook_index=self.hook_index;
		self.hook_index+=1;
		let a:Rc<dyn Any>=ci.hook_data[use_hook_index].clone();

		a.downcast::<RefCell<T>>().unwrap()
	}
}
