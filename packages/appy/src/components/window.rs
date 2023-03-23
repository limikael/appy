use std::rc::Rc;
use sdl2::event::{Event, WindowEvent};
use crate::{*};

pub struct GlWindowInstance {
	sdl: sdl2::Sdl,
	window: sdl2::video::Window,
	_gl_context: sdl2::video::GLContext,
	_video_subsystem: sdl2::VideoSubsystem,
	pub rect_renderer: RectRenderer,
	pub text_renderer: TextRenderer,
	event_listeners: Vec<Rc<dyn Fn(&Event)>>,
	pub rect: Rect
}

impl GlWindowInstance {
	fn new(props:Window)->GlWindowInstance {
		let sdl=sdl2::init().unwrap();
		let video_subsystem=sdl.video().unwrap();
		let window=video_subsystem
			.window(&props.title, props.init_width, props.init_height)
			.opengl()
			.resizable()
			.build()
			.unwrap();

		let gl_context=window.gl_create_context().unwrap();
		let _gl_loaded=gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

		let mut text_renderer=TextRenderer::new();
		text_renderer.window_width=props.init_width as i32;
		text_renderer.window_height=props.init_height as i32;

		let mut rect_renderer=RectRenderer::new();
		rect_renderer.window_width=props.init_width as i32;
		rect_renderer.window_height=props.init_height as i32;

		let rect=Rect{x:0, y:0, w:props.init_width as i32, h:props.init_height as i32};

		unsafe {
			gl::ClearColor(0.0,0.0,0.0,1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		};

		//println!("****** window opened, opengl is available...");

		Self {
			sdl,
			window,
			_video_subsystem: video_subsystem,
			_gl_context: gl_context,
			rect_renderer: rect_renderer,
			text_renderer,
			event_listeners: vec![],
			rect
		}
	}
}

pub fn use_gl_window_event(listener: Rc<dyn Fn(&Event)>) {
	let instance=use_context::<GlWindowInstance>();
	instance.borrow_mut().event_listeners.push(listener);
}

#[derive(Clone)]
pub struct Window {
	pub title: String,
	pub init_width: u32,
	pub init_height: u32
}

impl Default for Window {
	fn default()->Window {
		Self {
			title: "Untitled".to_string(),
			init_width: 640,
			init_height: 480
		}
	}
}

#[function_component]
pub fn window(props:Window, children:Elements)->Elements {
	//println!("render window!!");

	let instance_ref=use_instance(move||{
		GlWindowInstance::new(props.clone())
	});

	instance_ref.borrow_mut().event_listeners=vec![];
	use_context_provider(instance_ref.clone());

	let quit_trigger=use_quit_trigger();
	let dirty_trigger=use_dirty_trigger();

	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT);
	};

	use_post_render(Rc::new(with_clone!([instance_ref],move||{
		instance_ref.borrow_mut().window.gl_swap_window();
	})));

	use_idle(Rc::new(with_clone!([instance_ref],move||{
		let mut instance=instance_ref.borrow_mut();
		let mut event_pump=instance.sdl.event_pump().unwrap();
		let e=event_pump.wait_event();
		for listener in &instance.event_listeners {
			listener(&e);
		}

		match e {
			Event::Window {win_event: WindowEvent::Resized(x,y), ..} => {
				//println!("resize..");
				instance.rect_renderer.window_width=x.try_into().unwrap();
				instance.rect_renderer.window_height=y.try_into().unwrap();
				instance.text_renderer.window_width=x.try_into().unwrap();
				instance.text_renderer.window_height=y.try_into().unwrap();
				instance.rect=Rect{x:0, y:0,
					w: x.try_into().unwrap(),
					h: y.try_into().unwrap(),
				};
				unsafe { gl::Viewport(0,0,x,y) };
				dirty_trigger();
			},

			Event::Quit {..} => {
				quit_trigger();
			},
			_ => {},
		}
	})));

	children
}
