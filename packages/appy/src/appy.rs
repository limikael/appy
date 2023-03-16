use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
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
	signal_handlers: Vec<SignalHandler>
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
		this_path.push(ComponentPathComponent::TypeId(component.get_type_id()));

		if !self.instances.contains_key(&this_path) {
			let c=ComponentInstance::new();
			self.instances.insert(this_path.clone(),Rc::new(RefCell::new(c)));
		}

		let ci=self.instances.get(&this_path).unwrap().clone();
		let (child_fragment, mut signal_handlers)=
			RenderEnv::render(component,ci);

		self.signal_handlers.append(&mut signal_handlers);

		self.render_fragment(child_fragment,this_path);
	}

	fn render(&mut self) {
		self.signal_handlers=vec![];
		self.render_fragment(self.root_fragment.clone(),vec![]);
	}

	fn run_post_render(&mut self) {
		for handler in &self.signal_handlers {
			match handler {
				SignalHandler::PostRender(f)=>f(),
				_=>{}
			}
		}
	}

	fn run_idle(&mut self)->IdleAction {
		loop {
			for handler in &self.signal_handlers {
				match handler {
					SignalHandler::Idle(f)=>{
						let res=f();

						if res!=IdleAction::None {
							return res
						}
					},
					_=>{}
				}
			}
		}
	}

	pub fn run(fragment: ComponentFragment) {
		let mut appy=Self{
			instances: HashMap::new(),
			root_fragment: fragment,
			signal_handlers: vec![]
		};

		loop {
			appy.render();
			appy.run_post_render();
			if appy.run_idle()==IdleAction::Quit {
				break;
			}
		}
	}
}