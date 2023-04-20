use appy::{*, types::*, hooks::*};
use std::rc::Rc;

/// Show an image in the current block.
///
/// Example:
/// ```rust
/// #[main_window]
/// fn main()->Elements {
///     let img_src=use_state(||ImageSource::from_memory(include_bytes!("myimage.png")));
///
///     apx!{
///         <img src=img_src.as_rc()/>
///     }
/// }
/// ```
#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Img {
	src: Option<Rc<ImageSource>>,
	scale_mode: ScaleMode,
	align: Align,
	valign: VAlign,
}

#[function_component]
fn _img_render(props:Img)->Elements {
	let app_context=use_context::<AppContext>();
	if props.src.is_none() {
		return apx!{}
	}

	let tx=props.src.unwrap();
	let r=&app_context.rect;

	let target_aspect=r.w as f32/r.h as f32;
	let src_aspect=tx.width as f32/tx.height as f32;

	let size=match props.scale_mode {
		ScaleMode::Fit=>
			if src_aspect>target_aspect {
				(r.w as f32,r.w as f32/src_aspect)
			} else {
				(r.h as f32*src_aspect,r.h as f32)
			},

		ScaleMode::Fill=>
			if src_aspect<target_aspect {
				(r.w as f32,r.w as f32/src_aspect)
			} else {
				(r.h as f32*src_aspect,r.h as f32)
			},

		ScaleMode::None=>
			(tx.width as f32*app_context.pixel_ratio,tx.height as f32*app_context.pixel_ratio)
	};

	let x=match props.align {
		Align::Left => r.x,
		Align::Center => r.x+(r.w-size.0 as i32)/2,
		Align::Right => r.x+r.w-size.0 as i32,
	};

	let y=match props.valign {
		VAlign::Top => r.y,
		VAlign::Middle => r.y+(r.h-size.1 as i32)/2,
		VAlign::Bottom => r.y+r.h-size.1 as i32,
	};

	let r=Rect{x,y,w:size.0 as i32,h:size.1 as i32};

	app_context.image_renderer.borrow().draw(&r,&*tx);

	apx!{}
}