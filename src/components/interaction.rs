use crate::hooks::{use_app_event, use_context, use_state, StateRef};
use crate::rc_with_clone;
use crate::types::*;
use crate::types::{AppContext, Elements};
use appy::{derive_component, function_component, ComponentBuilder, SnakeFactory};
use glapp::{AppEvent, MouseKind};
use std::rc::Rc;

/// Checks interaction for the current [`Blk`](crate::components::Blk).
///
/// When the area is clicked, the closure specified with `on_click` will be called.
///
/// If you want to continously check the hover state (e.g. to render a button
/// with different states for hover and activation), use the
/// [`use_hover_state_ref`](crate::hooks::use_hover_state_ref()) hook, in conjuction with setting
/// the hover_state_ref prop.
#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct Interaction {
    on_click: Option<Rc<dyn Fn()>>,
    hover_state_ref: Option<StateRef<HoverState>>,
}

#[function_component]
fn _interaction(props: Interaction) -> Elements {
    let h_state = use_state(|| HoverState::Normal);
    let app_context = use_context::<AppContext>();
    let rect = app_context.rect.clone();

    if props.hover_state_ref.is_some() && *h_state != **props.hover_state_ref.as_ref().unwrap() {
        panic!("they are different!!!");
    }

    let hover_state_ref = props.hover_state_ref.clone();
    let update_h_state = rc_with_clone!([h_state, hover_state_ref], move |new_state| {
        h_state.set(new_state);
        if hover_state_ref.is_some() {
            hover_state_ref.as_ref().unwrap().set(new_state);
        }
    });

    let on_click = props.on_click.clone();
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
                        if on_click.is_some() {
                            (on_click.as_ref().unwrap())();
                        }
                    }

                    match kind {
                        MouseKind::Touch => update_h_state(HoverState::Normal),
                        MouseKind::Mouse => update_h_state(HoverState::Hover),
                    }
                }
            }
            AppEvent::MouseMove { x, y, kind, .. } => {
                if rect.contains(*x, *y) && *h_state == HoverState::Normal {
                    match kind {
                        MouseKind::Touch => {}
                        MouseKind::Mouse => update_h_state(HoverState::Hover),
                    }
                } else if !rect.contains(*x, *y) && *h_state != HoverState::Normal {
                    update_h_state(HoverState::Normal);
                }
            }
            _ => {}
        }
    }));

    props.children
}
