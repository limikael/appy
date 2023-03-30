use crate::*;
use sdl2::event::{Event, WindowEvent};
use std::rc::Rc;

type EventListeners = Vec<Rc<dyn Fn(&Event)>>;

pub struct GlWindowInstance {
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    _video_subsystem: sdl2::VideoSubsystem,
    pub rect_renderer: RectRenderer,
    pub text_renderer: TextRenderer,
    event_listeners: EventListeners,
    pub rect: Rect,
    first_render: bool,
}

impl GlWindowInstance {
    fn new(props: Window) -> GlWindowInstance {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let init_size = {
            if video_subsystem.display_orientation(0) == sdl2::video::Orientation::Unknown {
                props.desktop_init_size
            } else {
                let usable_bounds = video_subsystem.display_usable_bounds(0).unwrap();
                (usable_bounds.width(), usable_bounds.height())
            }
        };

        let window = video_subsystem
            .window(&props.title, init_size.0, init_size.1)
            .opengl()
            .resizable()
            .hidden()
            .build()
            .unwrap();

        let init_size = window.size();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        //install_debug_output();

        let mut text_renderer = TextRenderer::new();
        text_renderer.window_width = init_size.0 as i32;
        text_renderer.window_height = init_size.1 as i32;

        let mut rect_renderer = RectRenderer::new();
        rect_renderer.window_width = init_size.0 as i32;
        rect_renderer.window_height = init_size.1 as i32;

        let rect = Rect {
            x: 0,
            y: 0,
            w: init_size.0 as i32,
            h: init_size.1 as i32,
        };

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, init_size.0 as i32, init_size.1 as i32);
        };

        Self {
            sdl,
            window,
            _video_subsystem: video_subsystem,
            _gl_context: gl_context,
            rect_renderer,
            text_renderer,
            event_listeners: vec![],
            rect,
            first_render: true,
        }
    }
}

pub fn use_gl_window_event(listener: Rc<dyn Fn(&Event)>) {
    let instance = use_context::<GlWindowInstance>();
    instance.borrow_mut().event_listeners.push(listener);
}

#[derive(Clone)]
pub struct Window {
    pub title: String,
    pub desktop_init_size: (u32, u32),
}

impl Default for Window {
    fn default() -> Window {
        Self {
            title: "Untitled".to_string(),
            desktop_init_size: (640, 480),
        }
    }
}

#[function_component]
pub fn window(props: Window, children: Elements) -> Elements {
    let instance_ref = use_instance(move || GlWindowInstance::new(props.clone()));

    instance_ref.borrow_mut().event_listeners = vec![];
    use_context_provider(instance_ref.clone());

    let quit_trigger = use_quit_trigger();
    let dirty_trigger = use_dirty_trigger();

    //let instance=instance_ref.borrow();

    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    };

    use_post_render(Rc::new(with_clone!([instance_ref], move || {
        let mut instance = instance_ref.borrow_mut();

        if instance.first_render {
            instance.window.show();
            instance.first_render = false;
        }

        instance.window.gl_swap_window();
    })));

    use_idle(Rc::new(with_clone!([instance_ref], move || {
        let mut instance = instance_ref.borrow_mut();
        let mut event_pump = instance.sdl.event_pump().unwrap();
        let e = event_pump.wait_event();
        //log_debug!("{:?}",e);

        for listener in &instance.event_listeners {
            listener(&e);
        }

        match e {
            Event::Window {
                win_event: WindowEvent::Resized(x, y),
                ..
            } => {
                instance.rect_renderer.window_width = x;
                instance.rect_renderer.window_height = y;
                instance.text_renderer.window_width = x;
                instance.text_renderer.window_height = y;
                instance.rect = Rect {
                    x: 0,
                    y: 0,
                    w: x,
                    h: y,
                };
                unsafe { gl::Viewport(0, 0, x, y) };
                dirty_trigger();

                // shouldn't be needed, but makes it work on android...
                instance.window.gl_swap_window();
            }

            Event::Window {
                win_event: WindowEvent::Exposed,
                ..
            } => {
                dirty_trigger();
            }

            Event::Window {
                win_event: WindowEvent::Restored,
                ..
            } => {
                dirty_trigger();
            }

            Event::Quit { .. } => {
                quit_trigger();
            }
            _ => {}
        }
    })));

    children
}
