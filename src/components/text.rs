use appy::{*, hooks::*, types::*, utils::*};
use std::rc::Rc;

/// Render text.
///
/// Renders text in the current [Blk](crate::components::Blk).
///
/// The alignment inside the `blk` can be speficied with the align and valign
/// props.
///
/// In order to use custom fonts, first get a reference to the font using
/// [`use_font_data`](crate::hooks::use_font_data).
///
/// Example:
/// ```
/// use appy::{*, hooks::*, components::*, types::*};
///
/// #[main_window]
/// fn main()->Elements {
///	    let font=use_font_data(||include_bytes!("../core/Roboto-Regular.ttf"));
///
///	    apx!{
///		    <Text text="Hello World" font=font size=100/>
///	    }
/// }
/// ```
#[derive_component(ComponentBuilder, SnakeFactory)]
pub struct Text {
    color: u32,
    text: String,
    align: Align,
    valign: VAlign,
    font: Option<Rc<Font>>,
    size: Dim,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            color: 0xffffff,
            text: "<text>".to_string(),
            align: Align::Center,
            valign: VAlign::Middle,
            children: vec![],
            key: None,
            font: Option::<Rc<Font>>::None,
            size: Dim::Absolute(20.0),
        }
    }
}

#[function_component]
fn _text(props: Text) -> Elements {
    let app_context = use_context::<AppContext>();
    let r = &app_context.rect;

    let font = props.font.unwrap_or(app_context.default_font.clone());
    let size = props.size.to_abs(app_context.rect.h);
    let w = font.get_str_width(&props.text, size);

    let x = match props.align {
        Align::Left => r.x,
        Align::Center => r.x + (r.w - w) / 2.0,
        Align::Right => r.x + r.w - w,
    };

    let y = match props.valign {
        VAlign::Top => r.y,
        VAlign::Middle => r.y + (r.h - size) / 2.0,
        VAlign::Bottom => r.y + r.h - size,
    };

    let spec=TextRendererSpec{
        text: &props.text,
        x,
        y,
        font: &font,
        size,
        col: props.color,
        pr: app_context.pixel_ratio,
        alpha: app_context.alpha
    };

    app_context.text_renderer.borrow_mut().draw(&spec);

    props.children
}
