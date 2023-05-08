use appy::{components::*, hooks::*, types::*, *};

/// Positions a block in a flow layout
///
/// Will place items in a flow. Left to right, then top to bottom.
#[derive_component(ComponentBuilder, SnakeFactory)]
pub struct Flow {
    pub width: Dim,
    pub height: Dim,
}

impl Default for Flow {
    fn default() -> Self {
        Self {
            width: pct(100.0),
            height: pct(100.0),
            children: vec![],
            key: None,
        }
    }
}

#[function_component]
fn _flow(mut props: Flow) -> Elements {
    let app_context = use_context::<AppContext>();
    props.width = Dim::Absolute(props.width.to_abs(app_context.rect.w));
    props.height = Dim::Absolute(props.height.to_abs(app_context.rect.h));
    app_context.flow_elements.borrow_mut().push(props);

    vec![]
}

impl Flow {
    pub fn make_block(self: Self, x: f32, y: f32) -> ElementWrap<Blk> {
        blk()
            .left(x)
            .top(y)
            .width(self.width.clone())
            .height(self.height.clone())
            .key_option(self.get_key())
            .children(self.children)
    }
}
