//use crate::{*};
//use std::rc::Rc;
//use std::any::Any;

pub trait ElementT {
	fn render(self: Box<Self>)->Elements;
}

pub struct Element<T> {
	props: T,
	renderer: fn(T, Elements)->Elements,
	children: Elements,
}

impl<T: 'static> Element<T> {
	pub fn call_render(self: Box<Self>)->Elements {
		(self.renderer)(self.props,self.children)
	}

	pub fn create(renderer: fn(T, Elements)->Elements, props: T, children: Elements)->Box<dyn ElementT> {
		Box::new(Self{renderer,props,children})
	}
}

impl<T: 'static> ElementT for Element<T> {
	fn render(self: Box<Self>)->Elements {
		self.call_render()
//		vec![]
	}
}

pub type Elements=Vec<Box<dyn ElementT>>;
