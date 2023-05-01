use appy::{*, types::*, hooks::*};

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

    let w=props.width.to_abs(app_context.rect.w);
    let h=props.height.to_abs(app_context.rect.h);

    //app_context.flow_anchor.borrow_mut().add_children(w as i32, h as i32, props.children);
    app_context.flow_bucket.borrow_mut().add(props.children,w,h);

    vec![]
}
