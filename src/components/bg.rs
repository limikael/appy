use crate::utils::RectRendererSpec;
use appy::{hooks::*, types::*, *};

/// Draws a single colored rectangle, filling the current [Blk](crate::components::Blk).
///
/// It is intented to be used inside [crate::apx], e.g.:
///
/// ```rust
/// use appy::{*, components::*};
///
/// apx!{
///   <bg color=0x112233 />
///	};
///
#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct Bg {
    color: u32,
    border_color: u32,
    corner_radius: Dim,
    border_width: Dim,
    borders: Option<[bool; 4]>,
}

#[function_component]
fn _bg(props: Bg) -> Elements {
    let app_context = use_context::<AppContext>();

    let rect_renderer_spec = RectRendererSpec {
        viewport_size: app_context.viewport_size,
        rect: app_context.rect.clone(),
        col: props.color,
        border_col: props.border_color,
        corner_radius: props.corner_radius.to_abs(0.),
        border_width: props.border_width.to_abs(0.),
        borders: props.borders.unwrap_or([true, true, true, true]),
    };

    app_context.rect_renderer.draw(&rect_renderer_spec);
    vec![]
}
