use appy::{function_component,derive_component,SnakeFactory,ComponentBuilder};
use crate::hooks::use_context;
use crate::types::{*};

/// Render text.
///
/// Renders text in the current [Blk](crate::components::Blk).
///
/// The alignment inside the `blk` can be speficied with the align and valign
/// props.
///
/// The vertical size of the text can be specified using a `Dim`, meaning
/// that it can be specified in an absolute number or as a percentage relative to
/// the parent.
#[derive_component(ComponentBuilder,SnakeFactory)]
pub struct Text {
	col: u32,
	size: Dim,
	text: String,
	align: Align,
	valign: VAlign
}

impl Default for Text {
	fn default()->Self {
		Self {
			col: 0xffffff,
			size: Dp(16.0),
			text: "<text>".to_string(),
			align: Align::Center,
			valign: VAlign::Middle,
			children: vec![],
			key: None
		}
	}
}

#[function_component]
fn _text(props:Text)->Elements {
	let app_context=use_context::<AppContext>();
	let r=&app_context.rect;

	let size=app_context.compute_v_px(props.size);//.to_px(r.h as f32,app_context.pixel_ratio);
	let w=app_context.text_renderer.borrow().get_str_width(&props.text,size) as i32;

	let x=match props.align {
		Align::Left => r.x,
		Align::Center => r.x+(r.w-w)/2,
		Align::Right => r.x+r.w-w,
	};

	let y=match props.valign {
		VAlign::Top => r.y,
		VAlign::Middle => r.y+(r.h-size as i32)/2,
		VAlign::Bottom => r.y+r.h-size as i32,
	};

	app_context.text_renderer.borrow_mut().draw(&props.text, x as f32, y as f32, size, props.col);

	props.children
}
