use std::rc::Rc;
use std::cell::RefCell;
//use std::any::Any;
use std::any::TypeId;
use crate::{*};

pub fn use_instance<F, T: 'static>(ctor: F)->Rc<RefCell<T>>
		where F:Fn()->T {
	let env_ref=RenderEnv::get_current();
	if env_ref.borrow().have_current_hook_data() {
		return env_ref.borrow_mut().get_current_hook_data_no_ctor::<T>();
	}

	let data=ctor();
	let mut env=env_ref.borrow_mut();
	env.create_current_hook_data(data)
}

pub struct RefData<T> {
	pub current: T
}

pub fn use_ref<F, T: 'static>(ctor: F)->Rc<RefCell<RefData<T>>>
		where F:Fn()->T {
	use_instance(||RefData{current:ctor()})
}

pub struct StateData<T> {
	pub state_value: Rc<RefCell<T>>
}

pub fn use_state<F, T: 'static>(ctor: F)->(Rc<RefCell<T>>,Rc<dyn Fn(T)>)
		where F:Fn()->T {
	let dirty_trigger=use_dirty_trigger();
	let state_data_ref=use_instance(||StateData{
		state_value: Rc::new(RefCell::new(ctor()))
	});

	let state_data=state_data_ref.borrow();
	(
		state_data.state_value.clone(),
		{
			let state_value=state_data.state_value.clone();
			Rc::new(move|value:T|{
				state_value.replace(value);
				dirty_trigger();
			})
		}
	)

//	(state_data.state_value.clone(),Rc::new(move|value:T|{
//		state_data.value=Rc::new(RefCell::new(value));
//	}))
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	RenderEnv::get_current().borrow_mut().post_render_handlers.push(f);
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