use crate::{*};

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
	pub size: f32,
	pub text: &'static str,
	pub align: Align,
	pub valign: VAlign
}

impl Default for Text {
	fn default()->Self {
		Self {
			col: 0xffffff,
			size: 24.0,
			text: &"<text>",
			align: Align::Left,
			valign: VAlign::Middle
		}
	}
}

#[function_component]
pub fn text(p: Text, children: Elements)->Elements {
	let instance_ref=use_context::<GlWindowInstance>();
	let mut instance=instance_ref.borrow_mut();
	let r=instance.rect.clone();
	let w=instance.text_renderer.get_str_width(p.text,p.size) as i32;

	let x=match p.align {
		Align::Left => r.x,
		Align::Center => r.x+(r.w-w)/2,
		Align::Right => r.x+r.w-w,
	};

	let y=match p.valign {
		VAlign::Top => r.y,
		VAlign::Middle => r.y+(r.h-p.size as i32)/2,
		VAlign::Bottom => r.y+r.h-p.size as i32,
	};

	instance.text_renderer.draw(p.text, x as f32, y as f32, p.size, p.col);

	children
}
