mod app_context;
pub use app_context::*;

mod dim;
pub use dim::*;
//pub use dim::Dim::{Pc,Px,Dp};

mod element;
pub use element::*;

mod rect;
pub use rect::*;

mod image_source;
pub use image_source::*;

mod font;
pub use font::*;

/// Define how an image should scale in relation to its container.
#[derive(Default, Clone, Copy)]
pub enum ScaleMode {
    #[default]
    Fit,
    Fill,
    None,
}

/// Specify a horizontal alignment.
#[derive(Default, Clone, Debug, Copy)]
pub enum Align {
    Left,

    #[default]
    Center,
    Right,
}

/// Specify a vertical alignment.
#[derive(Default, Clone, Debug, Copy)]
pub enum VAlign {
    Top,

    #[default]
    Middle,
    Bottom,
}

/// Hold information about the hovered state of a component.
#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum HoverState {
    #[default]
    Normal,
    Hover,
    Active,
}
