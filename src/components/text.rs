use appy_macros::function_component;

use crate::appy::app_context::AppContext;
use crate::appy::element::Elements;
use crate::appy::hooks::use_context;

use super::blk::Dim;

#[derive(Clone)]
pub enum Align {
	Left,
	Center,
	Right
}

#[derive(Clone)]
pub enum VAlign {
	Top,
	Middle,
	Bottom
}

#[derive(Clone)]
pub struct Text {
	pub col: u32,
	pub size: Dim,
	pub text: String,
	pub align: Align,
	pub valign: VAlign
}

impl Default for Text {
	fn default()->Self {
		Self {
			col: 0xffffff,
			size: Dim::Px(24.0),
			text: "<text>".to_string(),
			align: Align::Left,
			valign: VAlign::Middle
		}
	}
}

//todo!("percentage height");

#[function_component]
pub fn text(p: Text, children: Elements)->Elements {
	let instance_ref=use_context::<AppContext>();
	let mut instance=instance_ref.borrow_mut();
	let r=instance.rect.clone();

	let size=p.size.to_px(r.h as f32,instance.pixel_ratio);
	let w=instance.text_renderer.get_str_width(&p.text,size) as i32;

	let x=match p.align {
		Align::Left => r.x,
		Align::Center => r.x+(r.w-w)/2,
		Align::Right => r.x+r.w-w,
	};

	let y=match p.valign {
		VAlign::Top => r.y,
		VAlign::Middle => r.y+(r.h-size as i32)/2,
		VAlign::Bottom => r.y+r.h-size as i32,
	};

	instance.text_renderer.draw(&p.text, x as f32, y as f32, size, p.col);

	children
}
