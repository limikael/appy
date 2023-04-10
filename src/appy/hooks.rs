use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
//use std::any::Any;
use std::any::TypeId;
use crate::{*};

pub fn use_instance<F, T: 'static>(ctor: F)->Rc<RefCell<T>>
		where F:Fn()->T {
	RenderEnv::use_hook_data(&|_env:&mut RenderEnv|{
		RefCell::new(ctor())
	})
}

/*pub struct RefData<T> {
	pub current: T
}

pub fn use_ref<F, T: 'static>(ctor: F)->Rc<RefCell<RefData<T>>>
		where F:Fn()->T {
	use_instance(||RefData{current:ctor()})
}*/

pub struct StateData<T> {
	pub value: Rc<T>,
	pub dirty_trigger: Rc<dyn Fn()>
}

#[derive(Clone)]
pub struct StateRef<T> {
	data: Rc<RefCell<StateData<T>>>,
	value: Rc<T>,
}

impl<T> StateRef<T> {
	pub fn new(data: Rc<RefCell<StateData<T>>>)->Self {
		let value=data.borrow().value.clone();
		Self {
			value,
			data
		}
	}

    pub fn set(&self, value: T) {
		self.data.borrow_mut().value=Rc::new(value);
		(self.data.borrow().dirty_trigger)();
	}
}

impl<T> Deref for StateRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

pub fn use_state<F, T: 'static>(ctor: F)->StateRef<T>
		where F:Fn()->T {
	StateRef::new(
		RenderEnv::use_hook_data(|env|{
			RefCell::new(StateData{
				value: Rc::new(ctor()),
				dirty_trigger: env.dirty.create_trigger(),
			})
		})
	)
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	let ci_ref=RenderEnv::get_current().borrow_mut().get_current_component_instance();
	ci_ref.borrow_mut().post_render=Some(f);
}

pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
	RenderEnv::get_current().borrow_mut().app_event_handlers.push(f);
}

pub fn use_dirty_trigger()->Rc<dyn Fn()> {
	let t=RenderEnv::use_hook_data(|env| {
		RefCell::new(
			env.dirty.create_trigger()
		)
	});

	let f=t.borrow().clone();
	f
}

/*pub fn use_context_provider<T: 'static>(t: Rc<RefCell<T>>) {
	let type_id=TypeId::of::<T>();

	if RenderEnv::get_current().borrow().contexts.contains_key(&type_id) {
		panic!("context already provided");
	}

	RenderEnv::get_current().borrow_mut().contexts.insert(type_id,t);
}*/

pub fn use_context<T: 'static>()->Rc<RefCell<T>> {
	let type_id=TypeId::of::<T>();

	let any=RenderEnv::get_current().borrow_mut().contexts.get(&type_id).unwrap().clone();
	any.downcast::<RefCell<T>>().unwrap()
}
