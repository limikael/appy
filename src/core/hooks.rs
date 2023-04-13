use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;

use crate::sys::app_window::AppEvent;

use super::Appy;
use super::component::HookRef;

#[derive(Clone)]
pub struct ReducerRef<T, A> {
	hook_ref: HookRef<T>,
	reducer: Rc<dyn Fn(&T,A)->T>
}

impl<T: 'static, A> ReducerRef<T, A> {
	pub fn dispatch(&self, action: A) {
		let reduced:T=(self.reducer)(&*self.hook_ref,action);
		self.hook_ref.set(reduced);
	}
}

impl<T, A> Deref for ReducerRef<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.hook_ref
    }
}

pub fn use_reducer<F, G, A: 'static, T: 'static>(reducer: G, ctor: F)->ReducerRef<T, A>
		where F:Fn()->T, G: Fn(&T,A)->T + 'static {
	Appy::with(|appy|{
		ReducerRef {
			hook_ref: appy.use_hook_ref(ctor),
			reducer: Rc::new(reducer)
		}
	})
}

pub type StateRef<T>=HookRef<T>;
pub fn use_state<F, T: 'static>(ctor: F)->StateRef<T>
		where F:Fn()->T {
	Appy::with(|appy|{
		appy.use_hook_ref(ctor)
	})
}

pub fn use_post_render(f: Rc<dyn Fn()>) {
	Appy::with(|appy|{
		appy.with_current_component_instance(|ci|{
			ci.post_render=Some(f.clone());
		})
	})
}

pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
	Appy::with(|appy|{
		appy.app_event_handlers.push(f.clone());
	})
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
