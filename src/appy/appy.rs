use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use environmental::environmental;
use crate::*;

environmental!(appy_instance:Appy);

pub struct Appy {
    instances: HashMap<ComponentPath, Rc<RefCell<ComponentInstance>>>,
    root: fn() -> Elements,
    app_context: Option<Rc<RefCell<AppContext>>>,
    pub current_component_instance: Option<Rc<RefCell<ComponentInstance>>>,
    current_hook_index: usize,
    pub app_event_handlers: Vec<Rc<dyn Fn(&AppEvent)>>,
    pub dirty: Trigger,
    pub contexts: HashMap<TypeId, Rc<dyn Any>>,
}

impl Appy {
    pub fn with<F, T: 'static>(mut f:F)->T
            where F: FnMut(&mut Appy)->T {
        appy_instance::with(|appy|{
            f(appy)
        }).unwrap()
    }

    pub fn use_hook_ref<F, T: 'static>(mut ctor:F)->HookRef<T>
            where F: FnMut()->T {
        appy_instance::with(move|appy|{
            let ci_ref = appy.current_component_instance.clone().unwrap();
            let mut ci = ci_ref.borrow_mut();

            appy.current_hook_index+=1;

            ci.get_hook_ref(
                appy.current_hook_index-1,
                move||ctor(),
                appy.dirty.create_trigger()
            )
        }).unwrap()
    }

    fn render_fragment(&mut self, fragment: Elements, component_path: ComponentPath) {
        for (i, component) in fragment.into_iter().enumerate() {
            let mut this_path = component_path.clone();
            this_path.push(ComponentPathComponent::Index(i as i32));

            self.render_component(component, this_path);
        }
    }

    fn render_component(&mut self, component: Box<dyn ElementT>, component_path: ComponentPath) {
        let mut this_path = component_path;
        this_path.push(ComponentPathComponent::TypeId(component.type_id()));

        if !self.instances.contains_key(&this_path) {
            self.instances.insert(
                this_path.clone(), 
                Rc::new(RefCell::new(ComponentInstance::new()))
            );
        }

        let ci = self.instances.get(&this_path).unwrap().clone();

        ci.borrow_mut().post_render = None;
        self.current_component_instance = Some(ci.clone());
        self.current_hook_index = 0;
        let child_fragment=appy_instance::using(self,||{
            component.render()
        });

        self.render_fragment(child_fragment,this_path);
        ci.borrow().run_post_render();
    }

    fn provide_context<T: 'static>(&mut self, t: Rc<RefCell<T>>) {
        let type_id=TypeId::of::<T>();

        if self.contexts.contains_key(&type_id) {
            panic!("context already provided");
        }

        self.contexts.insert(type_id,t);
    }

    fn render(&mut self) {
        self.app_event_handlers=vec![];
        self.contexts = HashMap::new();
        self.dirty.set_state(false);

        self.provide_context(self.app_context.clone().unwrap());
        self.render_component(
            Element::create(root_element, RootElement { root: self.root }, vec![]),
            vec![],
        );
    }

    pub fn new(root: fn() -> Elements)->Self {
        Self {
            root,
            instances: HashMap::new(),
            app_context: None,
            app_event_handlers: vec![],
            contexts: HashMap::new(),
            dirty: Trigger::new(),
            current_component_instance: None,
            current_hook_index: 0
        }
    }

    fn update_app_context_size(&mut self, w:i32, h:i32) {
        let ac_ref=self.app_context.clone().unwrap();
        let mut ac=ac_ref.borrow_mut();

        ac.set_size(w,h);
    }

    pub fn run(mut self, app_window_builder:&mut dyn AppWindowBuilder) {
        let app_window=app_window_builder.build();

        app_window.run(Box::new(move|w,e|{
            //log_debug!("app: {:?}",e);

            for handler in &self.app_event_handlers {
                handler(&e);
            }

            match e {
                AppEvent::Show=>{
                    //install_debug_output();
                    if self.app_context.is_none() {
                        self.app_context=Some(Rc::new(RefCell::new(AppContext::new())));
                        self.update_app_context_size(w.width() as i32,w.height() as i32);
                    }
                },
                AppEvent::Resize{width,height}=>{
                    self.update_app_context_size(width as i32,height as i32);
                }
                AppEvent::Render=>{
                    //println!("render");
                    self.render();
                },
                _=>{}
            }

            if self.dirty.get_state() {
                w.post_redisplay();
            }
        }));
    }
}
