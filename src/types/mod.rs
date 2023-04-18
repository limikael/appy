mod app_context;
pub use app_context::*;

mod dim;
pub use dim::*;
pub use dim::Dim::{Pc,Px,Dp};

mod element;
pub use element::*;

mod rect;
pub use rect::*;

/// Specify a horizontal alignment.
#[derive(Default,Clone)]
pub enum Align {
	Left,

	#[default]
	Center,
	Right
}

/// Specify a vertical state.
#[derive(Default,Clone)]
pub enum VAlign {
	Top,

	#[default]
	Middle,
	Bottom
}

/// Hold information about the hovered state of a component.
#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum HoverState {
    #[default]
    Normal,
    Hover,
    Active,
}

