use crate::hooks::{use_animation_frame, use_state, HookRef};
use crate::rc_with_clone;
use std::ops::Deref;

#[derive(Clone)]
pub enum SpringConf {
    Spring {
        stiffness: f32,
        damping: f32,
        epsilon: f32,
    },
    Linear {
        velocity: f32,
    },
}

impl SpringConf {
    pub const DEFAULT: SpringConf = SpringConf::spring(170.0, 26.0);
    pub const STIFF: SpringConf = SpringConf::spring(210.0, 20.0);

    pub const fn spring(stiffness: f32, damping: f32) -> Self {
        Self::Spring {
            stiffness,
            damping,
            epsilon: 0.001,
        }
    }

    pub const fn linear(velocity: f32) -> Self {
        Self::Linear { velocity: velocity }
    }

    pub fn epsilon(self: Self, new_epsilon: f32)->Self {
        match self {
            Self::Spring{stiffness,damping,epsilon:_}=>{
                Self::Spring {
                    stiffness,
                    damping,
                    epsilon: new_epsilon,
                }
            },
            Self::Linear{velocity:_}=>{
                panic!("no epsilon for linear")
            }
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
    fn tick(&self, conf: &SpringConf, delta: f32) -> Self {
        match *conf {
            SpringConf::Spring {
                stiffness,
                damping,
                epsilon: _,
            } => {
                fn dampened_hooke_force(
                    displacement: f32,
                    velocity: f32,
                    stiffness: f32,
                    damping: f32,
                ) -> f32 {
                    let hooke_force = -1.0 * (stiffness * displacement);
                    hooke_force - (damping * velocity)
                }

                let mut spring: SpringData = self.clone();
                let displacement = spring.current - spring.target;
                let force = dampened_hooke_force(displacement, spring.velocity, stiffness, damping);

                spring.velocity += force * delta;
                spring.current += spring.velocity * delta;

                if spring.is_at_rest(&conf) {
                    spring.velocity=0.0;
                    spring.current=spring.target;
                }

                spring
            }
            SpringConf::Linear { velocity } => {
                let mut spring: SpringData = self.clone();

                if spring.current - spring.target < -velocity * delta {
                    spring.current += velocity * delta;
                } else if spring.current - spring.target > velocity * delta {
                    spring.current -= velocity * delta
                } else {
                    spring.current = spring.target
                }

                spring
            }
        }
    }

    fn is_at_rest(&self, conf: &SpringConf) -> bool {
        match *conf {
            SpringConf::Spring {
                stiffness: _,
                damping: _,
                epsilon,
            } => {
                if (self.current - self.target).abs() > epsilon || self.velocity.abs() > epsilon {
                    false
                } else {
                    true
                }
            }
            SpringConf::Linear { velocity:_ } => self.current == self.target,
        }
    }
}

#[derive(Clone)]
pub struct SpringRef {
    hook_ref: HookRef<SpringData>,
}

impl Deref for SpringRef {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &(&*self.hook_ref).current
    }
}

impl SpringRef {
    pub fn target(&self, v: f32) {
        //println!("setting target: {}",v);

        let mut d = (*self.hook_ref).clone();
        d.target = v;
        self.hook_ref.set(d);
    }

    pub fn set(&self, v: f32) {
        let mut d = (*self.hook_ref).clone();
        d.current = v;
        d.target = v;
        d.velocity = 0.0;
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
pub fn use_spring<F>(ctor: F, conf: SpringConf) -> SpringRef
where
    F: Fn() -> f32,
{
    let h = use_state(|| {
        let initial_value = ctor();
        SpringData {
            current: initial_value,
            target: initial_value,
            velocity: 0.0,
        }
    });

    if !h.get_inner_value().is_at_rest(&conf) {
        use_animation_frame(rc_with_clone!([h], move |delta| {
            //println!("delta: {:?}", delta);
            h.set(h.get_inner_value().tick(&conf, delta));
        }));
    }

    SpringRef { hook_ref: h }
}
