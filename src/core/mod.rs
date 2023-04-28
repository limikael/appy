use std::time::SystemTime;
use std::mem::take;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use environmental::environmental;
use std::time::Instant;

use glapp::{App,AppEvent};
use crate::utils::Trigger;

use crate::types::AppContext;
use self::component::ComponentInstance;
use self::component::ComponentPath;
use self::component::ComponentPathComponent;
use self::component::HookRef;
use crate::types::{Element, Elements, Font};
use crate::core::element::root_element;
use crate::components::context_provider;

#[doc(hidden)]
pub mod component;

#[doc(hidden)]
pub mod element;

environmental!(appy_instance:Appy);

#[doc(hidden)]
pub struct Appy {
    instances: HashMap<ComponentPath, ComponentInstance>,
    previous_instances: HashMap<ComponentPath, ComponentInstance>,
    root: fn() -> Elements,
    app_context: Option<Rc<AppContext>>,
    current_hook_index: usize,
    current_component_path: Option<ComponentPath>,
    last_render: Option<SystemTime>,
    pub app_event_handlers: Vec<Rc<dyn Fn(&AppEvent)>>,
    pub animation_frame_handlers: Vec<Rc<dyn Fn(f32)>>,
    pub dirty: Trigger,
    pub contexts: HashMap<TypeId, Vec<Rc<dyn Any>>>,
}

impl Appy {
    pub fn with<F, T: 'static>(f:F)->T
            where F: FnOnce(&mut Appy)->T {
        appy_instance::with(|appy|{
            f(appy)
        }).unwrap()
    }

    pub fn use_hook_ref<F, T: 'static>(&mut self, ctor:F)->HookRef<T>
            where F: FnOnce()->T {
        let i=self.current_hook_index;
        let t=self.dirty.create_trigger();

        self.current_hook_index+=1;
        self.with_current_component_instance(|ci|{
            ci.create_hook_ref(i,ctor,t)
        })
    }

    pub fn with_current_component_instance<F, T: 'static>(&mut self, f:F)->T
           where F: FnOnce(&mut ComponentInstance)->T {
        let p=self.current_component_path.as_ref().unwrap().clone();
        if !self.instances.contains_key(&p) {
            let ci=if self.previous_instances.contains_key(&p) {
                self.previous_instances.remove(&p).unwrap()
            } else {
                ComponentInstance::new()
            };

            self.instances.insert(p.clone(),ci);
        }

        let ci = self.instances.get_mut(&p).unwrap();
        f(ci)
    }

    fn render_fragment(&mut self, fragment: Elements, component_path: ComponentPath) {
        for (i, element) in fragment.into_iter().enumerate() {
            let mut this_path = component_path.clone();

            let key=element.get_key();
            this_path.push(
                if key.is_some() {
                    ComponentPathComponent::Key(key.unwrap())
                } else {
                    ComponentPathComponent::Index(i as i32)
                }
            );

            self.render_component(element, this_path);
        }
    }

    fn render_component(&mut self, component: Box<dyn Element>, component_path: ComponentPath) {
        let mut this_path = component_path;
        this_path.push(ComponentPathComponent::TypeId(component.type_id()));

        if self.instances.contains_key(&this_path) {
            self.instances.get_mut(&this_path).unwrap().pre_render();
        }

        self.current_component_path=Some(this_path.clone());
        self.current_hook_index = 0;
        let child_fragment=appy_instance::using(self,||{
            component.render()
        });

        self.current_component_path=None;
        self.render_fragment(child_fragment,this_path.clone());

        if self.instances.contains_key(&this_path) {
            let post_render=&self.instances.get(&this_path).unwrap().post_render;
            if post_render.is_some() {
                let cb=post_render.clone().unwrap();
                appy_instance::using(self,||{
                    cb();
                });
            }
        }
    }

    fn render(&mut self) {
        self.app_event_handlers=vec![];
        self.animation_frame_handlers=vec![];
        self.contexts = HashMap::new();
        self.dirty.set_state(false);

        self.previous_instances=take(&mut self.instances);
        self.instances=HashMap::new();

        self.app_context.as_ref().unwrap().reset_flow(); //flow_anchor.borrow_mut()=(0,0);

        self.render_component(
            context_provider()
                .value(self.app_context.clone().unwrap())
                .children(vec![
                    root_element().root(self.root)
                ]),
            vec![]
        );

        self.previous_instances=HashMap::new();

        if self.animation_frame_handlers.len()>0 {
            let now=SystemTime::now();
            let delta=if self.last_render.is_some() {
                let duration=now.duration_since(self.last_render.unwrap()).unwrap();
                duration.as_millis() as f64/1000.0
            } else {0.0};

            //println!("delta: {:?}",delta);

            for handler in &self.animation_frame_handlers {
                handler(delta as f32);
            }

            self.last_render=if self.dirty.get_state() {
                Some(now)
            } else {
                None
            }
        } else {
            self.last_render=None
        }

        //println!("instances post render: {}",self.instances.len());
        //println!("dirty after render: {}",self.dirty.get_state());
    }

    pub fn new(root: fn() -> Elements)->Self {
        Self {
            root,
            last_render: None,
            instances: HashMap::new(),
            previous_instances: HashMap::new(),
            app_context: None,
            app_event_handlers: vec![],
            animation_frame_handlers: vec![],
            contexts: HashMap::new(),
            dirty: Trigger::new(),
            current_component_path: None,
            current_hook_index: 0
        }
    }

    pub fn run(mut self, app: App) {
        app.run(move|w,e|{
            //log_debug!("app: {:?}",e);

            for handler in &self.app_event_handlers {
                handler(&e);
            }

            match e {
                AppEvent::Show=>{
                    //install_debug_output();
                    if self.app_context.is_none() {
                        let size=w.size();
                        self.app_context=Some(Rc::new(AppContext::new(
                            size.0,size.1,
                            w.pixel_ratio(),
                            Font::from_data(
                                include_bytes!("./Roboto-Regular.ttf")
                            )
                        )));
                    }
                },
                AppEvent::Resize{width,height}=>{
                    let new_context=self.app_context.as_ref().unwrap().resize(
                        width as i32,
                        height as i32,
                        w.pixel_ratio()
                    );

                    self.app_context=Some(Rc::new(new_context));
                }
                AppEvent::Render=>{
                    let start = Instant::now();
                    self.render();
                    let duration = start.elapsed();
                    println!("Render time: {:?}", duration);
//                    self.render();
                },
                _=>{}
            }

            if self.dirty.get_state() {
                w.post_redisplay();
            }
        });
    }
}
