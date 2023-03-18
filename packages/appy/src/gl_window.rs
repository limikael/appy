use appy_macros::component;
use std::rc::Rc;
use sdl2::event::Event;
use crate::{*};

#[component]
pub struct GlWindow {
}

pub struct GlWindowInstance {
	sdl: sdl2::Sdl,
	window: sdl2::video::Window,
	_gl_context: sdl2::video::GLContext,
	_video_subsystem: sdl2::VideoSubsystem,
	pub rect_renderer: RectRenderer,
	event_listeners: Vec<Rc<dyn Fn(&Event)>>
}

impl GlWindowInstance {
	fn new()->GlWindowInstance {
		let sdl=sdl2::init().unwrap();
		let video_subsystem=sdl.video().unwrap();
		let window=video_subsystem
			.window("App", 800, 600)
			.opengl()
			.resizable()
			.build()
			.unwrap();

		let gl_context=window.gl_create_context().unwrap();
		let _gl_loaded=gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

		let mut rect_renderer=RectRenderer::new();
		rect_renderer.window_width=800;
		rect_renderer.window_height=600;

		unsafe {
			gl::ClearColor(0.0,0.0,0.0,1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		};

		Self {
			sdl,
			window,
			_video_subsystem: video_subsystem,
			_gl_context: gl_context,
			rect_renderer: rect_renderer,
			event_listeners: vec![]
		}
	}
}

pub fn use_gl_window_event(listener: Rc<dyn Fn(&Event)>) {
	let instance=use_context::<GlWindowInstance>();
	instance.borrow_mut().event_listeners.push(listener);
}

impl Component for GlWindow {
	fn render(&self)->ComponentFragment {
		let instance_ref=use_instance(||GlWindowInstance::new());
		instance_ref.borrow_mut().event_listeners=vec![];
		use_context_provider(instance_ref.clone());

		let quit_trigger=use_quit_trigger();

		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		};

		use_post_render(Rc::new(with_clone!([instance_ref],move||{
			instance_ref.borrow_mut().window.gl_swap_window();
		})));

		use_idle(Rc::new(with_clone!([instance_ref],move||{
			let instance=instance_ref.borrow_mut();
			let mut event_pump=instance.sdl.event_pump().unwrap();
			let e=event_pump.wait_event();
			for listener in &instance.event_listeners {
				listener(&e);
			}

			match e {
				Event::Quit {..} => {
					quit_trigger();
				},
				_ => {},
			}
		})));

		self.children.clone()
	}
}
