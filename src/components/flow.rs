use appy::{*, types::*, hooks::*, utils::*};

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
fn _flow(mut props:Flow)->Elements {
    let app_context = use_context::<AppContext>();
    props.width=Dim::Absolute(props.width.to_abs(app_context.rect.w));
    props.height=Dim::Absolute(props.height.to_abs(app_context.rect.h));

    app_context.flow_elements.borrow_mut().push(FlowElement{
        width: props.width.get_abs(), //app_context.rect.w),
        height: props.height.get_abs(), //app_context.rect.h),
        children: props.children,
        key: props.key
    });

    vec![]
}
