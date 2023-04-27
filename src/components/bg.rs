use appy::{*, types::*, hooks::*};
use crate::utils::RectRendererSpec;

/// Draws a single colored rectangle, filling the current [Blk](crate::components::Blk).
///
/// It is intented to be used inside [crate::apx], e.g.:
///
/// ```rust
/// apx!{
///   <bg col=0x112233 />
///	}
///
#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Bg {
	color: u32,
	border_color: u32,
	corner_radius: Dim,
	border_width: Dim,
	borders: Option<[bool; 4]>,
}

#[function_component]
fn _bg(props:Bg)->Elements {
	let app_context=use_context::<AppContext>();

	/*let mut borders:Vec<f32>=vec![];
	for i in 0..4 {
		borders.push(props.borders[i%props.borders.len()].to_px(0.0,app_context.pixel_ratio))
	}*/

	let rect_renderer_spec=RectRendererSpec{
		viewport_size: app_context.viewport_size,
		rect: Rect{
			x: app_context.rect.x as f32,
			y: app_context.rect.y as f32,
			w: app_context.rect.w as f32,
			h: app_context.rect.h as f32,
		},
		col: props.color,
		border_col: props.border_color,
		corner_radius: props.corner_radius.to_px(0.0,app_context.pixel_ratio),
		border_width: props.border_width.to_px(0.0,app_context.pixel_ratio),
		borders: props.borders.unwrap_or([true,true,true,true])
	};

	app_context.rect_renderer.draw(&rect_renderer_spec);
	vec![]
}
