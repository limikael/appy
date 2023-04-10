use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

/*#[derive(Clone)]
pub enum SignalHandler {
    PostRender(Rc<dyn Fn()>),
    Idle(Rc<dyn Fn()>),
}*/

#[derive(Default)]
pub struct ComponentInstance {
    hook_data: Vec<Rc<dyn Any>>,
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

thread_local! {
    static CURRENT_RENDER_ENV:RefCell<Option<Rc<RefCell<RenderEnv>>>>=RefCell::new(None);
}

#[derive(Default)]
pub struct RenderEnv {
    component_instance: Option<Rc<RefCell<ComponentInstance>>>,
    hook_index: usize,
    pub app_event_handlers: Vec<Rc<dyn Fn(&AppEvent)>>,
    pub dirty: Trigger,
    pub contexts: HashMap<TypeId, Rc<dyn Any>>,
}

impl RenderEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pre_render_tree(&mut self) {
        self.app_event_handlers=vec![];
        self.contexts = HashMap::new();
        self.dirty.set_state(false);
    }

    pub fn pre_render(&mut self, ci: Rc<RefCell<ComponentInstance>>) {
        ci.borrow_mut().post_render = None;
        self.component_instance = Some(ci);
        self.hook_index = 0;
    }

    pub fn post_render(&mut self) {
        self.component_instance = None;
    }

    pub fn get_current() -> Rc<RefCell<RenderEnv>> {
        CURRENT_RENDER_ENV.with(|instance| instance.borrow().clone().unwrap())
    }

    pub fn set_current(c: Option<Rc<RefCell<RenderEnv>>>) {
        CURRENT_RENDER_ENV.with(|instance| {
            *instance.borrow_mut() = c;
        });
    }

    pub fn get_current_component_instance(&self) -> Rc<RefCell<ComponentInstance>> {
        self.component_instance.clone().unwrap()
    }

    pub fn use_hook_data<T: 'static>(f:&dyn Fn(&mut RenderEnv)->T)->Rc<T> {
        let env_ref=RenderEnv::get_current();
        let mut env=env_ref.borrow_mut();

        env.use_hook_data_impl(f)
    }

    pub fn use_hook_data_impl<T: 'static>(&mut self, f:&dyn Fn(&mut RenderEnv)->T)->Rc<T> {
        let ci_ref = self.component_instance.clone().unwrap();
        let mut ci = ci_ref.borrow_mut();

        let use_hook_index=self.hook_index;
        if self.hook_index >= ci.hook_data.len() {
            ci.hook_data.push(Rc::new(f(self)))
        }

        self.hook_index += 1;

        let a:Rc<dyn Any>=ci.hook_data[use_hook_index].clone();
        a.downcast::<T>().unwrap()
    }

    pub fn provide_context<T: 'static>(&mut self, t: Rc<RefCell<T>>) {
        let type_id=TypeId::of::<T>();

        if self.contexts.contains_key(&type_id) {
            panic!("context already provided");
        }

        self.contexts.insert(type_id,t);
    }
}
