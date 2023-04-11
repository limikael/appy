use crate::*;
use std::rc::Rc;

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

#[derive(Clone, Default)]
pub struct Interaction {
    /*pub on_mouse_down: Cb,
    pub on_mouse_up: Cb,
    pub on_mouse_over: Cb,
    pub on_mouse_out: Cb,*/
    pub on_click: Cb,
    pub hover_state_ref: Option<StateRef<HoverState>>
}

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
