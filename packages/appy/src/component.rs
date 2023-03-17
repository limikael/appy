use std::rc::Rc;
//use crate::{*};

pub trait Component {
	fn render(&self)->ComponentFragment;
}

pub type ComponentFragment=Vec<Rc<dyn Component>>;
