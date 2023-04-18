use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use crate::sys::app_window::AppEvent;
use crate::core::Appy;
use crate::core::component::HookRef;
use crate::types::HoverState;

mod use_reducer;
pub use use_reducer::*;

mod use_spring;
pub use use_spring::*;

pub type StateRef<T>=HookRef<T>;

/// Track state in a function component.
pub fn use_state<F, T: 'static>(ctor: F)->StateRef<T>
		where F:Fn()->T {
	Appy::with(|appy|{
		appy.use_hook_ref(ctor)
	})
}

/// Post render handler.
///
/// The function specified will be called after the children of the
/// current component has been rendered.
pub fn use_post_render(f: Rc<dyn Fn()>) {
	Appy::with(|appy|{
		appy.with_current_component_instance(|ci|{
			ci.post_render=Some(f.clone());
		})
	})
}

/// Low level event handler.
///
/// Function handler for low level application events.
pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
	Appy::with(|appy|{
		appy.app_event_handlers.push(f.clone());
	})
}

/// Animation frame.
///
/// The registered function will be called after the next render.
pub fn use_animation_frame(f: Rc<dyn Fn(f32)>) {
	Appy::with(|appy|{
		appy.animation_frame_handlers.push(f.clone());
	})
}

/*pub fn use_context_provider<T: 'static>(t: Rc<RefCell<T>>) {
	let type_id=TypeId::of::<T>();

	if RenderEnv::get_current().borrow().contexts.contains_key(&type_id) {
		panic!("context already provided");
	}

	RenderEnv::get_current().borrow_mut().contexts.insert(type_id,t);
}*/

/// A context is a way to access global state.
pub fn use_context<T: 'static>()->Rc<RefCell<T>> {
	let type_id=TypeId::of::<T>();

	Appy::with(|appy|{
		let any=appy.contexts.get(&type_id).unwrap().clone();
		any.downcast::<RefCell<T>>().unwrap()
	})
}

/// Used to continously check the hover state of an interaction component.
///
/// For example:
/// ```
/// #[main_window]
/// pub fn app()->Elements {
///     let hover_state=use_hover_state_ref();
///     let col=match *hover_state {
///         HoverState::Normal=>0x808080,
///         HoverState::Active=>0x404040,
///         HoverState::Hover=>0xc0c0c0,
///     };
///
///     apx!{
///         <blk top=Pc(25.0) height=Pc(25.0) width=Pc(50.0) height=Pc(50.0)>
///             <bg col=col/>
///             <interaction hover_state_ref=Some(hover_state)/>
///         </blk>
///     }
/// }
/// ```
pub fn use_hover_state_ref()->StateRef<HoverState> {
    use_state(||HoverState::Normal)
}