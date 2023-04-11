use std::ops::Deref;
use std::cell::RefCell;
//use std::cell::Cell;
use std::any::TypeId;
use std::any::Any;
use std::rc::Rc;

#[derive(Clone)]
pub struct HookData {
    value: Rc<RefCell<Rc<dyn Any>>>
}

impl HookData {
    pub fn new(value: Rc<dyn Any>)->Self {
        HookData {
            value: Rc::new(RefCell::new(value))
        }
    }
}

#[derive(Clone)]
pub struct HookRef<T>
        where T: ?Sized {
    value: Rc<T>,
    hook_data: HookData,
    trigger: Rc<dyn Fn()>
}

impl<T: 'static> HookRef<T> {
    pub fn new(hook_data: HookData, trigger: Rc<dyn Fn()>)->Self {
        let any:Rc<dyn Any>=hook_data.value.borrow().clone();
        let value:Rc<T>=any.downcast::<T>().unwrap();

        HookRef {
            value,
            hook_data,
            trigger
        }
    }

    pub fn set(&self, v: T) {
        *self.hook_data.value.borrow_mut()=Rc::new(v);
        (self.trigger)();
    }
}

impl<T> Deref for HookRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

#[derive(Default)]
pub struct ComponentInstance {
    pub hook_data: Vec<HookData>,
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

    pub fn create_hook_ref<F, T: 'static>(
            &mut self, index:usize, mut ctor:F, 
            trigger:Rc<dyn Fn()>
        )->HookRef<T>
            where F:FnMut()->T {
        if index>self.hook_data.len() {
            panic!("Hooks are wrong");
        }

        if index==self.hook_data.len() {
            self.hook_data.push(HookData::new(Rc::new(ctor())))
        }

        HookRef::new(
            self.hook_data[index].clone(),
            trigger
        )
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ComponentPathComponent {
    Index(i32),
    TypeId(TypeId),
}

pub type ComponentPath = Vec<ComponentPathComponent>;
