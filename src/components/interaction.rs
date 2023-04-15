use appy_macros::function_component;

use crate::core::app_context::AppContext;
use crate::core::element::Elements;
use crate::core::hooks::{StateRef, use_state, use_context, use_app_event};
use crate::rc_with_clone;
use crate::sys::app_window::{AppEvent, MouseKind};
use crate::utils::cb::Cb;
use std::rc::Rc;

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

#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum HoverState {
    #[default]
    Normal,
    Hover,
    Active,
}

/// Props for the [`interaction`](interaction()) function component.
#[derive(Clone, Default)]
pub struct Interaction {
    /*pub on_mouse_down: Cb,
    pub on_mouse_up: Cb,
    pub on_mouse_over: Cb,
    pub on_mouse_out: Cb,*/
    pub on_click: Cb,
    pub hover_state_ref: Option<StateRef<HoverState>>
}

/// Checks interaction for the current [`blk`](crate::components::blk::blk()).
///
/// When the area is clicked, the closure specified with `on_click` will be called.
///
/// If you want to continously check the hover state (e.g. to render a button
/// with different states for hover and activation), use the
/// [`use_hover_state_ref`](use_hover_state_ref()) hook, in conjuction with setting
/// the hover_state_ref prop.
#[function_component]
pub fn interaction(p: Interaction, children: Elements) -> Elements {
    let h_state = use_state(|| HoverState::Normal);
    let instance_ref = use_context::<AppContext>();
    let rect = {
        let instance = instance_ref.borrow();
        instance.rect.clone()
    };

    if p.hover_state_ref.is_some() && *h_state!=**p.hover_state_ref.as_ref().unwrap() {
        panic!("they are different!!!");
    }

    let update_h_state = rc_with_clone!([h_state], move |new_state| {
        h_state.set(new_state);
        if p.hover_state_ref.is_some() {
            p.hover_state_ref.as_ref().unwrap().set(new_state);
        }
    });

    use_app_event(rc_with_clone!([], move |e| {
        match e {
            AppEvent::MouseDown { x, y, .. } => {
                if rect.contains(*x, *y) {
                    update_h_state(HoverState::Active);
                }
            }
            AppEvent::MouseUp { x, y, kind, .. } => {
                if rect.contains(*x, *y) {
                    if *h_state == HoverState::Active {
                        (p.on_click)();
                    }

                    match kind {
                        MouseKind::Touch=>update_h_state(HoverState::Normal),
                        MouseKind::Mouse=>update_h_state(HoverState::Hover)
                    }
                }
            }
            AppEvent::MouseMove { x, y, kind, .. } => {
                if rect.contains(*x, *y)
                    && *h_state == HoverState::Normal
                {
                    match kind {
                        MouseKind::Touch=>{},
                        MouseKind::Mouse=>update_h_state(HoverState::Hover)
                    }
                } else if !rect.contains(*x, *y) && *h_state != HoverState::Normal {
                    update_h_state(HoverState::Normal);
                }
            }
            _ => {}
        }
    }));

    children
}