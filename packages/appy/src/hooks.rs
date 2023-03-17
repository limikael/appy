use std::rc::Rc;
use std::cell::RefCell;
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
