use std::rc::Rc;
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
/// In order to draw text, use:
///
/// - [`use_font_face`](crate::hooks::use_font_face) - To get the data for the font.
/// - [`use_font`](crate::hooks::use_font) - To render the font to a texture for a specific size.
/// - [`Text`](crate::components::Text) - To render text on screen.
///
/// Example:
/// ```
///	let font_face=use_font_face(||include_bytes!("./Roboto-Regular.ttf"));
///	let font=use_font(font_face,100.0);
///
///	apx!{
///		<Text text="Hello World" font=font/>
///	}
/// ```
#[derive_component(ComponentBuilder,SnakeFactory)]
pub struct Text {
	color: u32,
	text: String,
	align: Align,
	valign: VAlign,
	font: Option<Rc<Font>>,
	size: Dim
}

impl Default for Text {
	fn default()->Self {
		Self {
			color: 0xffffff,
			text: "<text>".to_string(),
			align: Align::Center,
			valign: VAlign::Middle,
			children: vec![],
			key: None,
			font: Option::<Rc::<Font>>::None,
			size: Dim::Dp(20.0)
		}
	}
}

#[function_component]
fn _text(props:Text)->Elements {
	let app_context=use_context::<AppContext>();
	let r=&app_context.rect;

	let font=props.font.unwrap_or(app_context.default_font.clone());
	let size=app_context.compute_v_px(props.size);
	let w=font.get_str_width(&props.text,size) as i32;

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

	let mut tr=app_context.text_renderer.borrow_mut();
	tr.draw(&props.text,x as f32,y as f32,&font,size,props.color);

	props.children
}
