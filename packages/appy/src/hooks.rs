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

pub fn use_idle(f: Rc<dyn Fn()>) {
	use_signal(SignalHandler::Idle(f));
}

struct TriggerData {
	trigger: Rc<dyn Fn()>
}

pub fn use_quit_trigger()->Rc<dyn Fn()> {
	let env_ref=RenderEnv::get_current();
	if env_ref.borrow().have_current_hook_data() {
		let trigger_data=env_ref.borrow_mut().get_current_hook_data_no_ctor::<TriggerData>();
		return trigger_data.borrow().trigger.clone();
	}

	let d=TriggerData{trigger: env_ref.borrow().quit.create_trigger()};
	let trigger_data=env_ref.borrow_mut().create_current_hook_data(d);
	trigger_data.clone().borrow().trigger.clone()
}

pub fn use_dirty_trigger()->Rc<dyn Fn()> {
	let env_ref=RenderEnv::get_current();
	if env_ref.borrow().have_current_hook_data() {
		let trigger_data=env_ref.borrow_mut().get_current_hook_data_no_ctor::<TriggerData>();
		return trigger_data.borrow().trigger.clone();
	}

	let d=TriggerData{trigger: env_ref.borrow().dirty.create_trigger()};
	let trigger_data=env_ref.borrow_mut().create_current_hook_data(d);
	trigger_data.clone().borrow().trigger.clone()
}
