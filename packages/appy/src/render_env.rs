use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use crate::{*};

#[derive(Clone)]
pub enum SignalHandler {
	PostRender(Rc<dyn Fn()>),
	Idle(Rc<dyn Fn()>)
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
	pub post_render_handlers: Vec<Rc<dyn Fn()>>,
	pub idle_handlers: Vec<Rc<dyn Fn()>>,
	pub dirty: Trigger,
	pub quit: Trigger
}

impl RenderEnv {
	pub fn new()->Self {
		Self {
			component_instance: None,
			hook_index: 0,
			post_render_handlers: vec![],
			idle_handlers: vec![],
			dirty: Trigger::new(),
			quit: Trigger::new()
		}
	}

	pub fn pre_render_tree(&mut self) {
		self.post_render_handlers=vec![];
		self.idle_handlers=vec![];
	}

	pub fn pre_render(&mut self, ci:Rc<RefCell<ComponentInstance>>) {
		self.component_instance=Some(ci.clone());
		self.hook_index=0;
	}

	pub fn post_render(&mut self) {
		self.component_instance=None;
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

	pub fn get_current_hook_data_no_ctor<T: 'static>(&mut self)->Rc<RefCell<T>> {
		let ci_ref=self.component_instance.clone().unwrap();
		let ci=ci_ref.borrow();

		if self.hook_index>=ci.hook_data.len() {
			panic!("hook not found");
		}

		let use_hook_index=self.hook_index;
		self.hook_index+=1;
		let a:Rc<dyn Any>=ci.hook_data[use_hook_index].clone();

		a.downcast::<RefCell<T>>().unwrap()
	}

	pub fn have_current_hook_data(&self)->bool {
		let ci_ref=self.component_instance.clone().unwrap();
		let ci=ci_ref.borrow();

		if self.hook_index>=ci.hook_data.len() {
			return false;
		}

		true
	}

	pub fn create_current_hook_data<T: 'static>(&mut self, data:T)->Rc<RefCell<T>> {
		let ci_ref=self.component_instance.clone().unwrap();
		let mut ci=ci_ref.borrow_mut();

		if self.hook_index < ci.hook_data.len() {
			panic!("hook data already exists");
		}

		ci.hook_data.push(Rc::new(RefCell::new(data)));

		let use_hook_index=self.hook_index;
		self.hook_index+=1;
		let a:Rc<dyn Any>=ci.hook_data[use_hook_index].clone();

		a.downcast::<RefCell<T>>().unwrap()
	}
}
