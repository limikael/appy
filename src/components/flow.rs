use std::rc::Rc;
use appy::{*, types::*, components::*, hooks::*};

/// Positions a block in a flow layout
///
/// Will place items in a flow. Left to right, then top to bottom.
#[derive_component(ComponentBuilder,SnakeFactory)]
pub struct Flow {
    width: Dim,
    height: Dim,
}

impl Default for Flow {
    fn default()->Self {
        Self {
            width: pct(100.0),
            height: pct(100.0),
            children: vec![],
            key: None
        }
    }
}

#[function_component]
fn _flow(props:Flow)->Elements {
    let app_context = use_context::<AppContext>();

    let w=app_context.compute_h_px(props.width);
    let h=app_context.compute_v_px(props.height);
    let (x,y)=app_context.advance_flow(w as i32,h as i32);

    let new_context=app_context.abs(x,y,w as i32,h as i32);

    vec![
        context_provider()
            .value(Rc::new(new_context))
            .children(props.children)
    ]
}
