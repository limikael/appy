use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;

use crate::sys::app_window::AppEvent;

use super::Appy;
use super::component::HookRef;
use ::appy::utils::cb::CbP;
use ::appy::cb_p_with_clone;

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

/// Similar to the `use_state` hook, but passes the data through a reducer function.
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
pub fn use_animation_frame(f: CbP<f32>) {
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

#[derive(Clone)]
pub struct SpringConf {
	stiffness: f32,
	damping: f32,
	epsilon: f32
}

impl SpringConf {
	pub const DEFAULT:SpringConf=SpringConf::new(170.0,26.0);
	pub const STIFF:SpringConf=SpringConf::new(210.0,20.0);

	pub const fn new(stiffness:f32, damping:f32)->Self {
		Self {
			stiffness,
			damping,
			epsilon: 0.001
		}
	}
}

#[derive(Clone)]
struct SpringData {
	current: f32,
	velocity: f32,
	target: f32,
}

impl SpringData {
	fn tick(&self, conf:&SpringConf, delta:f32)->Self {
		fn dampened_hooke_force(displacement:f32, velocity:f32,
				stiffness:f32, damping:f32)->f32 {
			let hooke_force = -1.0 * (stiffness * displacement);
			hooke_force - (damping * velocity)
		}

		let mut spring:SpringData=self.clone();
		let displacement=spring.current-spring.target;
		let force=dampened_hooke_force(
			displacement,
			spring.velocity,
			conf.stiffness,
			conf.damping
		);

		spring.velocity+=force*delta;
		spring.current+=spring.velocity*delta;

		spring
	}
}

#[derive(Clone)]
pub struct SpringRef {
	hook_ref: HookRef<SpringData>
}

impl Deref for SpringRef {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &(&*self.hook_ref).current
    }
}

impl SpringRef {
	pub fn target(&self, v:f32) {
		let mut d=(*self.hook_ref).clone();
		d.target=v;
		self.hook_ref.set(d);
	}

	pub fn set(&self, v:f32) {
		let mut d=(*self.hook_ref).clone();
		d.current=v;
		d.target=v;
		d.velocity=0.0;
		self.hook_ref.set(d);
	}
}

pub fn use_spring<F>(ctor: F, conf: SpringConf)->SpringRef
		where F:Fn()->f32 {
	let h=use_state(||{
		let initial_value=ctor();
		SpringData{
			current: initial_value,
			target: initial_value,
			velocity: 0.0,
		}
	});

	if ((*h).current-(*h).target).abs()>conf.epsilon {
		use_animation_frame(cb_p_with_clone!([h],move|delta|{
			h.set((*h).tick(&conf,delta));
		}));
	}

	SpringRef {
		hook_ref: h
	}
}