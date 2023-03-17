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
	root_fragment: Vec<Rc<dyn Component>>,
	render_env: Rc<RefCell<RenderEnv>>
}

impl Appy {
	fn render_fragment(&mut self, fragment: ComponentFragment, component_path:ComponentPath) {
		let mut i=0;

		for component in fragment {
			let mut this_path=component_path.clone();
			this_path.push(ComponentPathComponent::Index(i));

			self.render_component(component,this_path);
			i+=1;
		}
	}

	fn render_component(&mut self, component: Rc<dyn Component>, component_path:ComponentPath) {
		let mut this_path=component_path.clone();
		this_path.push(ComponentPathComponent::TypeId(component.type_id()));

		if !self.instances.contains_key(&this_path) {
			let c=ComponentInstance::new();
			self.instances.insert(this_path.clone(),Rc::new(RefCell::new(c)));
		}

		let ci=self.instances.get(&this_path).unwrap().clone();

		self.render_env.borrow_mut().pre_render(ci);
		let child_fragment=component.render();
		self.render_env.borrow_mut().post_render();

		self.render_fragment(child_fragment,this_path);
	}

	fn render(&mut self) {
		self.render_env.borrow_mut().pre_render_tree();
		RenderEnv::set_current(Some(self.render_env.clone()));
		self.render_fragment(self.root_fragment.clone(),vec![]);
		RenderEnv::set_current(None);
	}

	fn run_post_render(&mut self) {
		let env=self.render_env.borrow();
		for handler in &env.signal_handlers {
			match handler {
				SignalHandler::PostRender(f)=>f(),
				_=>{}
			}
		}
	}

	fn run_idle(&mut self) {
		let env=self.render_env.borrow();
		for handler in &env.signal_handlers {
			match handler {
				SignalHandler::Idle(f)=>f(),
				_=>{}
			}
		}
	}

	pub fn run(fragment: ComponentFragment) {
		let mut appy=Self{
			instances: HashMap::new(),
			root_fragment: fragment,
			render_env: Rc::new(RefCell::new(RenderEnv::new()))
		};

		loop {
			appy.render_env.borrow().dirty.set_state(false);
			appy.render();
			appy.run_post_render();
			if appy.render_env.borrow().dirty.get_state() {
				panic!("dirty during render, unsupported for now");
			}

			while !appy.render_env.borrow().dirty.get_state() && 
					!appy.render_env.borrow().quit.get_state() {
				appy.run_idle();
			}

			if appy.render_env.borrow().quit.get_state() {
				break;
			}
		}
	}
}