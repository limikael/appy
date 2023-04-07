use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ComponentPathComponent {
    Index(i32),
    TypeId(TypeId),
}

type ComponentPath = Vec<ComponentPathComponent>;

pub struct Appy {
    instances: HashMap<ComponentPath, Rc<RefCell<ComponentInstance>>>,
    root: fn() -> Elements,
    render_env: Rc<RefCell<RenderEnv>>,
    app_context: Option<Rc<RefCell<AppContext>>>
}

impl Appy {
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
            let c = ComponentInstance::new();
            self.instances
                .insert(this_path.clone(), Rc::new(RefCell::new(c)));
        }

        let ci = self.instances.get(&this_path).unwrap().clone();

        self.render_env.borrow_mut().pre_render(ci.clone());
        let child_fragment = component.render();
        self.render_env.borrow_mut().post_render();

        self.render_fragment(child_fragment, this_path);

        ci.borrow().run_post_render();
    }

    fn render(&mut self) {
        self.render_env.borrow_mut().pre_render_tree();
        self.render_env.borrow_mut().provide_context::<AppContext>(self.app_context.clone().unwrap());

        RenderEnv::set_current(Some(self.render_env.clone()));

        self.render_component(
            Element::create(root_element, RootElement { root: self.root }, vec![]),
            vec![],
        );
        RenderEnv::set_current(None);
    }

    pub fn new(root: fn() -> Elements)->Self {
        Self {
            instances: HashMap::new(),
            root,
            render_env: Rc::new(RefCell::new(RenderEnv::new())),
            app_context: None,
        }
    }

    fn update_app_context_size(&mut self, w:i32, h:i32) {
        let ac_ref=self.app_context.clone().unwrap();
        let mut ac=ac_ref.borrow_mut();

        ac.rect.w=w;
        ac.rect.h=h;
        ac.rect_renderer.window_width=w;
        ac.rect_renderer.window_height=h;
        ac.text_renderer.window_width=w;
        ac.text_renderer.window_height=h;
    }

    pub fn run(mut self, app_window_builder:&mut dyn AppWindowBuilder) {
        let app_window=app_window_builder.build();

        app_window.run(Box::new(move|w,e|{
            //log_debug!("app: {:?}",e);

            for handler in &self.render_env.borrow().app_event_handlers {
                handler(&e);
            }

            match e {
                AppEvent::Show=>{
                    //install_debug_output();
                    if self.app_context.is_none() {
                        self.app_context=Some(Rc::new(RefCell::new(AppContext {
                            rect: Rect::empty(),
                            rect_renderer: RectRenderer::new(),
                            text_renderer: TextRenderer::new()
                        })));

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

            if self.render_env.borrow().dirty.get_state() {
                w.post_redisplay();
            }
        }));
    }
}
