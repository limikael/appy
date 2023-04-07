use std::ffi::{CString};
use std::num::NonZeroU32;

use winit::event::{Event, WindowEvent};
use winit::window::WindowBuilder;
use winit::event_loop::EventLoopBuilder;

use raw_window_handle::HasRawWindowHandle;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
//use glutin::surface::SwapInterval;

use glutin_winit::{self, DisplayBuilder, GlWindow};

use crate::*;

#[cfg(target_os="android")]
use winit::platform::android::EventLoopBuilderExtAndroid;

#[derive(Default)]
pub struct GlutinAppWindowBuilder {
    #[cfg(target_os="android")]
    android_app: Option<winit::platform::android::activity::AndroidApp>
}

impl AppWindowBuilder for GlutinAppWindowBuilder {
    fn build(&mut self)->Box<dyn AppWindow> {
        let mut event_loop_builder=EventLoopBuilder::new();

        #[cfg(target_os="android")] {
            event_loop_builder.with_android_app(self.android_app.take().unwrap());
        }

        Box::new(GlutinAppWindow::new(
            event_loop_builder.build(),
            "Hello".to_string()
        ))
    }
}

impl GlutinAppWindowBuilder {
    pub fn new()->Self {
        Self {..Default::default()}
    }

    #[cfg(target_os="android")]
    pub fn with_android_app(mut self, android_app:winit::platform::android::activity::AndroidApp)
            ->Self {
        self.android_app=Some(android_app);
        self
    }
}

pub struct GlutinAppWindow {
    gl_config: glutin::config::Config,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window: Option<winit::window::Window>,
    not_current_gl_context: Option<glutin::context::NotCurrentContext>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
    gl_surface: Option<glutin::surface::Surface<glutin::surface::WindowSurface>>,
    pub width: u32,
    pub height: u32,
    mouse_position: winit::dpi::PhysicalPosition<f64>
}

impl AppWindow for GlutinAppWindow {
    fn width(&self)->i32 {
        if self.width==0 {
            panic!("We have no window at this point");
        }
        self.width as i32
    }

    fn height(&self)->i32 {
        if self.height==0 {
            panic!("We have no window at this point");
        }
        self.height as i32
    }

    fn post_redisplay(&mut self) {
        //println!("request redraw...");
        self.window.as_ref().unwrap().request_redraw();
    }

    fn run(self: Box<Self>, handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
        self.run_impl(handler);
    }
}

impl GlutinAppWindow {
    pub fn new(event_loop:winit::event_loop::EventLoop<()>, _title:String)->Self {
        let window_builder=
            if cfg!(target_os = "android") {None}
            else {Some(WindowBuilder::new())};

        let template =
            ConfigTemplateBuilder::new();//.with_alpha_size(8);.with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                // Find the config with the maximum number of samples.
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Picked a config with {} samples", gl_config.num_samples());

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // XXX The display could be obtained from the any object created by it, so we
        // can query it from the config.
        let gl_display = gl_config.display();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                    |_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    },
                )
            })
        });

        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        //let inner_size=window.as_ref().unwrap().inner_size();

        GlutinAppWindow {
            gl_config,
            event_loop: Some(event_loop),
            window,
            not_current_gl_context,
            gl_context: None,
            gl_surface: None,
            width: 0, //inner_size.width,
            height: 0, //inner_size.height
            mouse_position: winit::dpi::PhysicalPosition::<f64>{x:0.0, y:0.0}
        }
    }

    fn run_impl(mut self, mut handler:Box<dyn FnMut(&mut dyn AppWindow,AppEvent)>) {
        let event_loop=self.event_loop.take().unwrap();

        event_loop.run(move |event, window_target, control_flow| {
            control_flow.set_wait();

            //println!("{:?}",event);
            match event {
                Event::Resumed => {
                    if self.window.is_none() {
                        let window_builder = WindowBuilder::new();
                        self.window=Some(glutin_winit::finalize_window(window_target, window_builder, &self.gl_config).unwrap());
                    }

                    let inner_size=self.window.as_ref().unwrap().inner_size();
                    self.width=inner_size.width;
                    self.height=inner_size.height;

                    let attrs = self.window.as_ref().unwrap().build_surface_attributes(<_>::default());
                    self.gl_surface=Some(unsafe {
                        self.gl_config.display().create_window_surface(&self.gl_config, &attrs).unwrap()
                    });

                    // Make it current.
                    self.gl_context =
                        Some(self.not_current_gl_context.take().unwrap().make_current(&self.gl_surface.as_ref().unwrap()).unwrap());

                    //assert!(state.replace((gl_surface)).is_none());
                    handler(&mut self,AppEvent::Show);

                    self.window.as_ref().unwrap().request_redraw();
                },
                Event::Suspended => {
                    // This event is only raised on Android, where the backing NativeWindow for a GL
                    // Surface can appear and disappear at any moment.
                    println!("Android window removed");

                    // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
                    // the window back to the system.
//                    let (gl_context, ..) = state.take().unwrap();
                    let not_current=self.gl_context.take().unwrap().make_not_current();

                    self.not_current_gl_context=Some(not_current.unwrap());
                    self.gl_context=None;
                    self.gl_surface=None;
                },
                /*Event::RedrawEventsCleared => {
                    if let Some((gl_context, gl_surface, window)) = &state {
                        handler(AppEvent::Render);
                        window.request_redraw();
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                },*/
                Event::RedrawRequested(_window_id)=>{
                    unsafe {
                        gl::ClearColor(0.0,0.0,0.0,0.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    handler(&mut self,AppEvent::Render);
                    self.gl_surface.as_ref().unwrap().swap_buffers(&self.gl_context.as_ref().unwrap()).unwrap();
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        self.gl_surface.as_ref().unwrap().resize(
                            &self.gl_context.as_ref().unwrap(),
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );

                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }

                        self.width=size.width;
                        self.height=size.height;
                        handler(&mut self,AppEvent::Resize{
                            width:size.width,
                            height:size.height
                        });

                        self.window.as_ref().unwrap().request_redraw();
                    },
                    WindowEvent::ScaleFactorChanged{new_inner_size,..}=>{
                        let size=new_inner_size;
                        self.gl_surface.as_ref().unwrap().resize(
                            &self.gl_context.as_ref().unwrap(),
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );

                        unsafe {
                            gl::Viewport(0, 0, size.width as i32, size.height as i32);
                        }

                        self.width=size.width;
                        self.height=size.height;
                        handler(&mut self,AppEvent::Resize{
                            width:size.width,
                            height:size.height
                        });

                        self.window.as_ref().unwrap().request_redraw();
                    },
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::CursorMoved{position,..}=>{
                        self.mouse_position=position;
                        handler(&mut self,AppEvent::MouseMove{
                            x:position.x as i32,
                            y:position.y as i32,
                            kind:MouseKind::Mouse
                        });
                    },
                    WindowEvent::MouseInput{state,button,..}=>{
                        let event_button=match button {
                            winit::event::MouseButton::Left=>MouseButton::Left,
                            winit::event::MouseButton::Right=>MouseButton::Right,
                            _=>MouseButton::Unknown
                        };

                        let event=match state {
                            winit::event::ElementState::Pressed=>{
                                AppEvent::MouseDown{
                                    x: self.mouse_position.x as i32,
                                    y: self.mouse_position.y as i32,
                                    kind: MouseKind::Mouse,
                                    button: event_button
                                }
                            },
                            winit::event::ElementState::Released=>{
                                AppEvent::MouseUp{
                                    x: self.mouse_position.x as i32,
                                    y: self.mouse_position.y as i32,
                                    kind: MouseKind::Mouse,
                                    button: event_button
                                }
                            }
                        };

                        handler(&mut self,event);
                    },
                    WindowEvent::Touch(winit::event::Touch{phase,location,..})=>{
                        let event=match phase {
                            winit::event::TouchPhase::Started=>{
                                AppEvent::MouseDown{
                                    x: location.x as i32,
                                    y: location.y as i32,
                                    kind: MouseKind::Touch,
                                    button: MouseButton::Unknown
                                }
                            },
                            winit::event::TouchPhase::Moved=>{
                                AppEvent::MouseMove{
                                    x: location.x as i32,
                                    y: location.y as i32,
                                    kind: MouseKind::Touch,
                                }
                            },
                            winit::event::TouchPhase::Ended |
                            winit::event::TouchPhase::Cancelled=>{
                                AppEvent::MouseUp{
                                    x: location.x as i32,
                                    y: location.y as i32,
                                    kind: MouseKind::Touch,
                                    button: MouseButton::Unknown
                                }
                            },
                        };

                        handler(&mut self,event);
                    }
                    _ => (),
                },
                _ => (),
            }
        });
	}
}
