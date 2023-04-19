use std::rc::Rc;
use std::any::TypeId;
use appy::*;
use appy::types::*;
use appy::hooks::use_post_render;
use crate::core::Appy;

/// Provide context.
///
/// Provide a context to be used together with [`use_context`](appy::hooks::use_context()).
#[derive_component(Default)]
pub struct ContextProvider<T> { 
	value : Option<Rc<T>>,
}

impl<T> ContextProvider<T> {
	pub fn new()->ElementWrap<ContextProvider<T>> {
		ElementWrap::new(Self {
			value: None,
			children: vec![],
			key: None
		})
	}

	pub fn children(mut self: ElementWrap<ContextProvider<T>>, children: Elements)
			->ElementWrap<ContextProvider<T>> {
		self.children=children;
		self
	}

	pub fn value(mut self: ElementWrap<ContextProvider<T>>, value: Rc<T>)
			->ElementWrap<ContextProvider<T>> {
		self.value=Some(value);
		self
	}
}

impl<T: 'static> Element for ContextProvider<T> {
	fn render(self:ElementWrap<Self>)->Elements {
		let t=self.value.clone().unwrap();

		Appy::with(|appy|{
	        let type_id=TypeId::of::<T>();

	        if !appy.contexts.contains_key(&type_id) {
	        	appy.contexts.insert(type_id,vec![]);
	        }

	        appy.contexts.get_mut(&type_id).unwrap().push(t);
		});

		use_post_render(Rc::new(||{
			Appy::with(|appy|{
		        let type_id=TypeId::of::<T>();

		        appy.contexts.get_mut(&type_id).unwrap().pop();
			});
		}));

		self.children
	}

	fn get_key(&self)->Option<String> {
		self.key.clone()
	}
}

/// Alias for ::new() on the corresponding struct.
pub fn context_provider<T>()->ElementWrap<ContextProvider<T>> {
	ContextProvider::<T>::new()
}
