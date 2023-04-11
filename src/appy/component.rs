use std::any::TypeId;
use std::any::Any;
use std::rc::Rc;

#[derive(Default)]
pub struct ComponentInstance {
    pub hook_data: Vec<Rc<dyn Any>>,
    pub post_render: Option<Rc<dyn Fn()>>,
}

impl ComponentInstance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_post_render(&self) {
        if self.post_render.is_some() {
            let f = self.post_render.as_ref().unwrap();
            (*f)();
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ComponentPathComponent {
    Index(i32),
    TypeId(TypeId),
}

pub type ComponentPath = Vec<ComponentPathComponent>;
