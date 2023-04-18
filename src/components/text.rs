use appy::{derive_component,SnakeFactory,ComponentBuilder};
use crate::hooks::use_context;
use crate::types::{*, Dim::*};

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
			children: vec![]
		}
	}
}

impl Element for Text {
	fn render(self:ElementWrap<Self>)->Elements {
		let instance_ref=use_context::<AppContext>();
		let mut instance=instance_ref.borrow_mut();
		let r=instance.rect.clone();

		let size=self.size.to_px(r.h as f32,instance.pixel_ratio);
		let w=instance.text_renderer.get_str_width(&self.text,size) as i32;

		let x=match self.align {
			Align::Left => r.x,
			Align::Center => r.x+(r.w-w)/2,
			Align::Right => r.x+r.w-w,
		};

		let y=match self.valign {
			VAlign::Top => r.y,
			VAlign::Middle => r.y+(r.h-size as i32)/2,
			VAlign::Bottom => r.y+r.h-size as i32,
		};

		instance.text_renderer.draw(&self.text, x as f32, y as f32, size, self.col);

		self.children
	}
}
