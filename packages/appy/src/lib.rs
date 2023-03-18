mod component;
pub use component::{*};

mod render_env;
pub use render_env::{*};

mod hooks;
pub use hooks::{*};

mod appy;
pub use crate::appy::{*};

mod gl_window;
pub use gl_window::{*};

mod gl_components;
pub use gl_components::{*};

mod utils;
pub use utils::{*};

mod gl_utils;
pub use gl_utils::{*};

pub use appy_macros::{*};

#[macro_export]
macro_rules! with_clone {
	([$ ($var:ident), *],$body:expr) => {
		{
			$(let $var=($var).clone();)*
			$body
		}
	}
}

#[cfg(test)]
mod tests;