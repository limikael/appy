use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use crate::{*};

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
	component_instance: Option<Rc<RefCell<ComponentInstance>>>,
	hook_index: usize,
	pub signal_handlers: Vec<SignalHandler>
}

impl RenderEnv {
	pub fn new()->Self {
		Self {
			component_instance: None,
			hook_index: 0,
			signal_handlers: vec![],
		}
	}

	pub fn render(&self, c: Rc<dyn Component>, ci: Rc<RefCell<ComponentInstance>>) {
		/*let re=RenderEnv{
			component_instance: ci,
			hook_index: 0,
			signal_handlers: vec![]
		};*/

		/*RenderEnv::set_current(Some(Rc::new(RefCell::new(re))));
		RenderEnv::set_current(None);*/

		let child_fragment=c.render();
		let signal_handlers=self.signal_handlers.clone();

		(child_fragment, signal_handlers)
	}

	pub fn get_current()->Rc<RefCell<RenderEnv>> {
		CURRENT_RENDER_ENV.with(|instance| {
			instance.borrow().clone().unwrap().clone()
		})
	}

	pub fn set_current(c: Option<Rc<RefCell<RenderEnv>>>) {
		CURRENT_RENDER_ENV.with(|instance| {
			*instance.borrow_mut()=c;
		});
	}

	pub fn get_current_hook_data<F, T: 'static>(&mut self, ctor: F)->Rc<RefCell<T>>
			where F: Fn()->T {
		let ci_ref=self.component_instance.clone().unwrap();
		let mut ci=ci_ref.borrow_mut();

		if self.hook_index>=ci.hook_data.len() {
			ci.hook_data.push(Rc::new(RefCell::new(ctor())));
		}

		let use_hook_index=self.hook_index;
		self.hook_index+=1;
		let a:Rc<dyn Any>=ci.hook_data[use_hook_index].clone();

		a.downcast::<RefCell<T>>().unwrap()
	}
}
