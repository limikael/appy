use appy_macros::component;
use std::rc::Rc;
use sdl2::event::Event;
use crate::{*};

#[component]
pub struct GlWindow {
}

struct GlWindowInstance {
	sdl: sdl2::Sdl,
	window: sdl2::video::Window,
	_gl_context: sdl2::video::GLContext,
	_video_subsystem: sdl2::VideoSubsystem
}

impl GlWindowInstance {
	pub fn new()->GlWindowInstance {
		println!("gl instance");

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

		unsafe {
			gl::ClearColor(0.0,0.0,0.0,1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		};

		Self {
			sdl,
			window,
			_video_subsystem: video_subsystem,
			_gl_context: gl_context
		}
	}
}

impl Component for GlWindow {
	fn render(&self)->ComponentFragment {
		let instance=use_instance(||GlWindowInstance::new());
		let quit_trigger=use_quit_trigger();
		let dirty_trigger=use_dirty_trigger();

		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		};

		let post_render_instance_ref=instance.clone();
		use_post_render(Rc::new(move||{
			let instance=post_render_instance_ref.borrow_mut();
			instance.window.gl_swap_window();
		}));

		let idle_instance_ref=instance.clone();
		use_idle(Rc::new(move||{
			let instance=idle_instance_ref.borrow_mut();
			let mut event_pump=instance.sdl.event_pump().unwrap();
			let e=event_pump.wait_event();
			match e {
				Event::Quit {..} => {
					quit_trigger();
				},
				Event::MouseButtonDown {/*x, y,*/ ..} => {
					dirty_trigger();
				},
				_ => {},
			}
		}));

		self.children.clone()
	}
}
