use std::rc::Rc;
use std::cell::RefCell;
//use std::any::Any;
use std::any::TypeId;
use crate::{*};

fn use_hook_data<F, T: 'static>(ctor: F)->Rc<T>
		where F:Fn()->Rc<T> {
	let env_ref=RenderEnv::get_current();
	if !env_ref.borrow().have_hook_data() {
		let data=ctor();
		env_ref.borrow_mut().create_hook_data(data);
	}

	return env_ref.borrow_mut().get_hook_data();
}

pub fn use_instance<F, T: 'static>(ctor: F)->Rc<RefCell<T>>
		where F:Fn()->T {
	use_hook_data(||Rc::new(RefCell::new(ctor())))
}

pub struct RefData<T> {
	pub current: T
}

pub fn use_ref<F, T: 'static>(ctor: F)->Rc<RefCell<RefData<T>>>
		where F:Fn()->T {
	use_instance(||RefData{current:ctor()})
}

pub struct StateData<T> {
	pub state_value: Rc<T>
}

pub fn use_state<F, T: 'static>(ctor: F)->(Rc<T>,Rc<dyn Fn(T)>)
		where F:Fn()->T {
	let dirty_trigger=use_dirty_trigger();
	let state_data_ref=use_instance(||StateData{
		state_value: Rc::new(ctor())
	});

	let current_value=state_data_ref.borrow().state_value.clone();
	(
		current_value,
		{
			Rc::new(move|value:T|{
				state_data_ref.borrow_mut().state_value=Rc::new(value);
				dirty_trigger();
			})
		}
	)
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	let ci_ref=RenderEnv::get_current().borrow_mut().get_current_component_instance();
	ci_ref.borrow_mut().post_render=Some(f);
}

pub fn use_idle(f: Rc<dyn Fn()>) {
	RenderEnv::get_current().borrow_mut().idle_handlers.push(f);
}

pub fn use_quit_trigger()->Rc<dyn Fn()> {
	use_instance(||{
		RenderEnv::get_current().borrow().quit.create_trigger()
	}).borrow().clone()
}

pub fn use_dirty_trigger()->Rc<dyn Fn()> {
	use_instance(||{
		RenderEnv::get_current().borrow().dirty.create_trigger()
	}).borrow().clone()
}

pub fn use_context_provider<T: 'static>(t: Rc<RefCell<T>>) {
	let type_id=TypeId::of::<T>();

	if RenderEnv::get_current().borrow().contexts.contains_key(&type_id) {
		panic!("context already provided");
	}

	RenderEnv::get_current().borrow_mut().contexts.insert(type_id,t);
}

pub fn use_context<T: 'static>()->Rc<RefCell<T>> {
	let type_id=TypeId::of::<T>();

	let any=RenderEnv::get_current().borrow_mut().contexts.get(&type_id).unwrap().clone();
	any.downcast::<RefCell<T>>().unwrap()
}
