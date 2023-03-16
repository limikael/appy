use std::rc::Rc;

use crate::{*};

pub trait Component: Typed {
	fn render(&self)->ComponentFragment;
}

pub type ComponentFragment=Vec<Rc<dyn Component>>;
