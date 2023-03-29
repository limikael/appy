use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::any::Any;
use std::collections::HashMap;

use crate::{*};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ComponentPathComponent {
	Index(i32),
	TypeId(TypeId)
}

type ComponentPath=Vec<ComponentPathComponent>;

pub struct Appy {
	instances: HashMap<ComponentPath,Rc<RefCell<ComponentInstance>>>,
	root: fn()->Elements,
	render_env: Rc<RefCell<RenderEnv>>
}

#[derive(Clone)]
pub struct RootElement {
	pub root: fn()->Elements
}

impl Default for RootElement {
	fn default()->Self {
		Self {
			root: ||{vec![]}
		}
	}
}

#[function_component]
pub fn root_element(p:RootElement, _c:Elements)->Elements {
	(p.root)()
}

impl Appy {
	fn render_fragment(&mut self, fragment: Elements, component_path:ComponentPath) {
		let mut i=0;

		for component in fragment {
			let mut this_path=component_path.clone();
			this_path.push(ComponentPathComponent::Index(i));

			self.render_component(component,this_path);
			i+=1;
		}
	}

	fn render_component(&mut self, component: Box<dyn ElementT>, component_path:ComponentPath) {
		let mut this_path=component_path.clone();
		this_path.push(ComponentPathComponent::TypeId(component.type_id()));

		if !self.instances.contains_key(&this_path) {
			let c=ComponentInstance::new();
			self.instances.insert(this_path.clone(),Rc::new(RefCell::new(c)));
		}

		let ci=self.instances.get(&this_path).unwrap().clone();

		self.render_env.borrow_mut().pre_render(ci.clone());
		let child_fragment=component.render();
		self.render_env.borrow_mut().post_render();

		self.render_fragment(child_fragment,this_path);

		ci.borrow().run_post_render();
	}

	fn render(&mut self) {
		self.render_env.borrow_mut().pre_render_tree();
		RenderEnv::set_current(Some(self.render_env.clone()));
//		self.render_fragment((self.root)(),vec![]);

		self.render_component(Element::create(root_element,RootElement{root:self.root},vec![]),vec![]);
		RenderEnv::set_current(None);
	}

	fn run_handlers(handlers:&Vec<Rc<dyn Fn()>>) {
		for handler in handlers {
			handler()
		}
	}

	fn render_loop(&mut self) {
		loop {
			self.render_env.borrow().dirty.set_state(false);
			self.render();
			//Self::run_handlers(&self.render_env.borrow().post_render_handlers);

			if self.render_env.borrow().dirty.get_state() {
				panic!("dirty during render, unsupported for now");
			}

			while !self.render_env.borrow().dirty.get_state() && 
					!self.render_env.borrow().quit.get_state() {
				Self::run_handlers(&self.render_env.borrow().idle_handlers);
			}

			if self.render_env.borrow().quit.get_state() {
				break;
			}
		}
	}

	pub fn run(root: fn()->Elements) {
		let mut appy=Self{
			instances: HashMap::new(),
			root: root,
			render_env: Rc::new(RefCell::new(RenderEnv::new()))
		};

		appy.render_loop();
	}
}