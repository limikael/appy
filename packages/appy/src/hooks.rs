use std::rc::Rc;
use std::cell::RefCell;
use crate::{*};

pub fn use_instance<F, T: 'static>(ctor: F)->Rc<RefCell<T>>
		where F:Fn()->T {
	let env_ref=RenderEnv::get_current();
	let mut env=env_ref.borrow_mut();

	env.get_current_hook_data(||ctor())
}

pub struct RefData<T> {
	pub current: T
}

pub fn use_ref<F, T: 'static>(ctor: F)->Rc<RefCell<RefData<T>>>
		where F:Fn()->T {
	use_instance(||RefData{current:ctor()})
}

pub fn use_signal(s: SignalHandler) {
	let env_ref=RenderEnv::get_current();
	let mut env=env_ref.borrow_mut();

	env.signal_handlers.push(s);
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	use_signal(SignalHandler::PostRender(f));
}

pub fn use_idle(f: Rc<dyn Fn()->IdleAction>) {
	use_signal(SignalHandler::Idle(f));
}

/*pub fn use_tick(f: Rc<dyn Fn()>) {
	let env_ref=RenderEnv::get_current();
	let mut env=env_ref.borrow_mut();

	env.tick_functions.push(f);
}*/