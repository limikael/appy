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
/// The vertical size of the text can be specified using a `Dim`, meaning
/// that it can be specified in an absolute number or as a percentage relative to
/// the parent.
#[derive_component(ComponentBuilder,SnakeFactory)]
pub struct Text {
	col: u32,
	text: String,
	align: Align,
	valign: VAlign,
	font: Option<Rc<Font>>
}

impl Default for Text {
	fn default()->Self {
		Self {
			col: 0xffffff,
			text: "<text>".to_string(),
			align: Align::Center,
			valign: VAlign::Middle,
			children: vec![],
			key: None,
			font: Option::<Rc::<Font>>::None
		}
	}
}

#[function_component]
fn _text(props:Text)->Elements {
	let app_context=use_context::<AppContext>();
	let r=&app_context.rect;

	if props.font.is_none() {
		return vec![]
	}

	let font=props.font.unwrap();
	let w=font.get_str_width(&props.text) as i32;

	let x=match props.align {
		Align::Left => r.x,
		Align::Center => r.x+(r.w-w)/2,
		Align::Right => r.x+r.w-w,
	};

	let y=match props.valign {
		VAlign::Top => r.y,
		VAlign::Middle => r.y+(r.h-font.size as i32)/2,
		VAlign::Bottom => r.y+r.h-font.size as i32,
	};

	let mut tr=app_context.text_renderer.borrow_mut();
	tr.draw(&props.text,x as f32,y as f32,&font,props.col);

	props.children
}
