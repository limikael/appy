use std::rc::Rc;
use std::any::TypeId;
use std::collections::HashMap;
use std::cell::RefCell;
use std::any::Any;

pub trait Typed {
	fn get_type_id(&self)->TypeId;
}

pub trait Component: Typed {
	fn render(&self)->ComponentFragment;
}

pub type ComponentFragment=Vec<Rc<dyn Component>>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ComponentPathComponent {
	Index(i32),
	TypeId(TypeId)
}

type ComponentPath=Vec<ComponentPathComponent>;

struct ComponentInstance {
	hook_index: usize,
	hook_data: Vec<Rc<dyn Any>>,
}

impl ComponentInstance {
	pub fn new()->Self {
		Self {
			hook_index: 0,
			hook_data: vec![]
		}
	}

	pub fn get_current_hook_data<F>(&mut self, ctor: F)->Rc<dyn Any>
			where F: Fn()->Rc<dyn Any> {
		if self.hook_index>=self.hook_data.len() {
			self.hook_data.push(ctor());
		}

		let use_hook_index=self.hook_index;
		self.hook_index+=1;
		self.hook_data[use_hook_index].clone()
	}
}

pub struct Appy {
	instances: HashMap<ComponentPath,Rc<RefCell<ComponentInstance>>>
}

thread_local! {
	static CURRENT_INSTANCE:RefCell<Option<Rc<RefCell<ComponentInstance>>>>=RefCell::new(None);
}

fn get_current_instance()->Rc<RefCell<ComponentInstance>> {
	CURRENT_INSTANCE.with(|instance| {
		instance.borrow().clone().unwrap().clone()
	})
}

fn set_current_instance(c: Option<Rc<RefCell<ComponentInstance>>>) {
	CURRENT_INSTANCE.with(|instance| {
		*instance.borrow_mut()=c;
	});
}

pub struct RefData<T> {
	pub current: T
}

pub fn use_ref<F, T: 'static>(ctor: F)->Rc<RefCell<RefData<T>>>
		where F:Fn()->T {
	let instance_ref=get_current_instance();
	let mut instance=instance_ref.borrow_mut();

	let hd_ref=instance.get_current_hook_data(||{
		Rc::new(RefCell::new(RefData{current: ctor()}))
	});

	hd_ref.downcast::<RefCell<RefData<T>>>().unwrap()
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
		ci.borrow_mut().hook_index=0;

		set_current_instance(Some(ci));
		let child_fragment=component.render();
		set_current_instance(None);

		self.render_fragment(child_fragment,this_path);
	}

	pub fn run(fragment: ComponentFragment) {
		let mut appy=Self{
			instances: HashMap::new()
		};

		appy.render_fragment(fragment.clone(),vec![]);
		appy.render_fragment(fragment.clone(),vec![]);

		println!("awef");
	}
}