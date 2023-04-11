use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
//use std::any::Any;
use std::any::TypeId;
use crate::{*};

pub fn use_instance<F, T: 'static>(ctor: F)->Rc<RefCell<T>>
		where F:Fn()->T {
	Appy::use_hook_data(&|_appy:&mut Appy|{
		RefCell::new(ctor())
	})
}

pub struct ReducerData<T> {
	pub value: Rc<T>,
	pub dirty_trigger: Rc<dyn Fn()>
}

#[derive(Clone)]
pub struct ReducerRef<T, A> {
	data: Rc<RefCell<ReducerData<T>>>,
	value: Rc<T>,
	reducer: Rc<dyn Fn(Rc<T>,A)->Rc<T>>
}

impl<T, A> ReducerRef<T, A> {
	pub fn new(data: Rc<RefCell<ReducerData<T>>>, reducer:Rc<dyn Fn(Rc<T>,A)->Rc<T>>)->Self {
		let value=data.borrow().value.clone();
		Self {
			value,
			data,
			reducer
		}
	}

	pub fn dispatch(&self, action: A) {
		let reduced:Rc<T>=(self.reducer)(self.value.clone(),action);
		self.data.borrow_mut().value=reduced;
		(self.data.borrow().dirty_trigger)();
	}
}

impl<T, A> Deref for ReducerRef<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

pub fn use_reducer<F, A, T: 'static>(reducer: Rc<dyn Fn(Rc<T>,A)->Rc<T>>, ctor: F)->ReducerRef<T, A>
		where F:Fn()->T {
	ReducerRef::new(
		Appy::use_hook_data(|env|{
			RefCell::new(ReducerData{
				value: Rc::new(ctor()),
				dirty_trigger: env.dirty.create_trigger(),
			})
		}),
		reducer
	)
}

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
		Appy::use_hook_data(|env|{
			RefCell::new(StateData{
				value: Rc::new(ctor()),
				dirty_trigger: env.dirty.create_trigger(),
			})
		})
	)
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	Appy::with(|appy|{
		let ci_ref=appy.current_component_instance.as_ref().unwrap();
		ci_ref.borrow_mut().post_render=Some(f.clone());
	})
}

pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
	Appy::with(|appy|{
		appy.app_event_handlers.push(f.clone());
	})
}

pub fn use_dirty_trigger()->Rc<dyn Fn()> {
	let t=Appy::use_hook_data(|appy| {
		RefCell::new(
			appy.dirty.create_trigger()
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

	Appy::with(|appy|{
		let any=appy.contexts.get(&type_id).unwrap().clone();
		any.downcast::<RefCell<T>>().unwrap()
	})
}
