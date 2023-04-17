use sdl2::event::{Event, WindowEvent};
use super::app_window::{MouseKind, MouseButton, AppWindowBuilder, AppWindow, AppEvent};

const SDL_TOUCH_MOUSEID: u32 = u32::MAX;

fn decode_mouse(mouse_id:u32, mouse_btn:sdl2::mouse::MouseButton)
		->(MouseKind,MouseButton) {
	if mouse_id==SDL_TOUCH_MOUSEID {
		return (MouseKind::Touch,MouseButton::Unknown)
	}

	match mouse_btn {
		sdl2::mouse::MouseButton::Left=>(MouseKind::Mouse,MouseButton::Left),
		sdl2::mouse::MouseButton::Right=>(MouseKind::Mouse,MouseButton::Right),
		_=>(MouseKind::Mouse,MouseButton::Unknown)
	}
}

pub struct SdlAppWindowBuilder {
	title: String
}

impl AppWindowBuilder for SdlAppWindowBuilder {
    fn build(&mut self)->Box<dyn AppWindow> {
    	Box::new(SdlAppWindow::new(self.title.clone()))
    }
}

impl SdlAppWindowBuilder {
	pub fn new()->Self {
		Self {
			title: "Unknown".to_string()
		}
	}

    pub fn title(mut self, title:String)
            ->Self {
        self.title=title;
        self
    }
}

pub struct SdlAppWindow {
	sdl: sdl2::Sdl,
	window: sdl2::video::Window,
	_gl_context: sdl2::video::GLContext,
	_video_subsystem: sdl2::VideoSubsystem,
	width: u32,
	height: u32,
	expose_requested: bool,
	quit_requested: bool,
	pixel_ratio: f32,
}

impl AppWindow for SdlAppWindow {
	fn size(&self)->(i32,i32) {
		(self.width as i32,self.height as i32)
	}

	fn pixel_ratio(&self)->f32 {
		self.pixel_ratio
	}

	fn post_redisplay(&mut self) {
		self.expose_requested=true;
	}

    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
    	self.run_impl(handler);
    }
}

impl SdlAppWindow {
	pub fn new(title:String)->Self {
		let sdl=sdl2::init().unwrap();
		let video_subsystem=sdl.video().unwrap();

		let mut pixel_ratio=1.0;
		let dpi=video_subsystem.display_dpi(0).unwrap().0;
		if dpi>160.0 {
			pixel_ratio=dpi/160.0;
		}

    	println!("pixel ratio: {:?}",pixel_ratio);

		let window=video_subsystem
			.window(&*title, 800, 600)
			.opengl()
			.resizable()
			.build()
			.unwrap();

		let gl_context=window.gl_create_context().unwrap();
		let _gl_loaded=gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let size = window.size();

		Self {
			sdl,
			window,
			_video_subsystem: video_subsystem,
			_gl_context: gl_context,
			width: size.0,
			height: size.1,
			expose_requested: false,
			pixel_ratio,
			quit_requested: false
		}
	}

	fn handle_event(&mut self, handler:&mut Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>, e:&Event) {
		//println!("{:?}",e);

		match *e {
			Event::Quit{..}=>{
				self.quit_requested=true;
			},
			/*Event::Window {win_event: WindowEvent::Shown, ..}=>{
				handler(&mut self,AppEvent::Show);
			},*/
			Event::Window {win_event: WindowEvent::Exposed, ..}=>{
				self.post_redisplay();
			}
			Event::Window {win_event: WindowEvent::Resized(w,h), ..}=>{
                unsafe { gl::Viewport(0, 0, w, h) };
                self.width=w as u32;
                self.height=h as u32;
                handler(self,AppEvent::Resize{
                	width:w as u32,
                	height:h as u32
                });

                // For some reason android need this extra render pass.
                self.do_render(handler);
                self.post_redisplay();

			}
			Event::Window {win_event: WindowEvent::Restored, ..}=>{
				self.post_redisplay();
			}
			Event::MouseButtonDown {x, y, mouse_btn, which, ..} => {
				let (kind,button)=decode_mouse(which,mouse_btn);
				handler(self,AppEvent::MouseDown{x,y,kind,button});
			}
			Event::MouseButtonUp {x, y, mouse_btn, which, ..} => {
				let (kind,button)=decode_mouse(which,mouse_btn);
				handler(self,AppEvent::MouseUp{x,y,kind,button});
			}
			Event::MouseMotion {x, y, which, ..} => {
				let (kind,_)=decode_mouse(which,sdl2::mouse::MouseButton::Unknown);
				handler(self,AppEvent::MouseMove{x,y,kind});
			}
			_ => {}
		}
	}

	fn do_render(&mut self, handler:&mut Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
		unsafe {
			gl::ClearColor(0.0,0.0,0.0,0.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		handler(self,AppEvent::Render);
		self.window.gl_swap_window();
	}

    fn run_impl(mut self, mut handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
		let mut event_pump=self.sdl.event_pump().unwrap();

		handler(&mut self,AppEvent::Show);
		self.expose_requested=true;

		while !self.quit_requested {
			let mut e=if self.expose_requested {
				event_pump.poll_event()
			} else {
				Some(event_pump.wait_event())
			};

			while e.is_some() {
				self.handle_event(&mut handler,&e.unwrap());
				e=event_pump.poll_event()
			}

			if self.expose_requested {
				self.expose_requested=false;
				self.do_render(&mut handler);
			}
		}
    }
}