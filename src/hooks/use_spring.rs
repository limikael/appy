use crate::hooks::{HookRef,use_state,use_animation_frame};
use crate::rc_with_clone;
use std::ops::Deref;

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

/// Animate value with spring physics.
///
/// You can think of `use_spring` as being similar to `use_state<f32>`. I.e.,
/// a numeric state that is being tracked for a function component. The
/// reference returned by this function also has a set method, similar
/// to `set_state`.
///
/// However, the reference also has a `target` method,
/// which will not set the state immediately, but rather set an internal
/// target, and then the value will smoothly change frame by frame towards
/// that target. For example:
/// ```
/// use appy::{*, types::*, components::*, hooks::*};
///
/// #[main_window]
/// pub fn app()->Elements {
///	   let x=use_spring(||0.0,SpringConf::DEFAULT);
///
///	    apx! {
///         // This will have the effect of animating smoothly.
///	        <blk right=10 bottom=10 height=90 width=150>
///		        <interaction on_click=rc_with_clone!([x],move||x.target(100.0))/>
///	        </blk>
///
///         // This will "hard" set the value, similar to set_state.
///	        <blk right=10 bottom=110 height=90 width=150>
///		        <interaction on_click=rc_with_clone!([x],move||x.set(100.0))/>
///	        </blk>
///
///	        <blk right=50>
///		        <blk left=pct(*x) width=50 height=50 top=50>
///			        <bg color=0xff0000/>
///		        </blk>
///	        </blk>
///     }
/// }
/// ```
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

	if ((*h).current-(*h).target).abs()>conf.epsilon ||
			(*h).velocity.abs()>conf.epsilon {
		use_animation_frame(rc_with_clone!([h],move|delta|{
			//println!("delta: {:?}",delta);
			h.set((*h).tick(&conf,delta));
		}));
	}

	SpringRef {
		hook_ref: h
	}
}