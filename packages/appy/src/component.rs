use std::rc::Rc;
use std::any::TypeId;

pub trait Typed {
	fn get_type_id(&self)->TypeId;
}

pub trait Component: Typed {
	fn render(&self)->ComponentFragment;
}

pub type ComponentFragment=Vec<Rc<dyn Component>>;
