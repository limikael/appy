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
use glutin::surface::SwapInterval;

use glutin_winit::{self, DisplayBuilder, GlWindow};

use crate::*;

pub struct AppWindow {
    gl_config: glutin::config::Config,
    event_loop: winit::event_loop::EventLoop<()>,
    window: Option<winit::window::Window>,
    not_current_gl_context: Option<glutin::context::NotCurrentContext>
}

impl AppWindow {
	pub fn new()->Self {
        let event_loop=EventLoopBuilder::new().build();

        let window_builder=
            if cfg!(target_os = "android") {None}
            else {Some(WindowBuilder::new())};

        let template =
            ConfigTemplateBuilder::new();

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

		AppLoop {
            gl_config,
            event_loop,
            window,
            not_current_gl_context
        }
	}

	pub fn run<F>(mut self, mut handler:F)->!
            where F: 'static + FnMut(AppEvent) {
        let mut state = None;

        self.event_loop.run(move |event, window_target, control_flow| {
            //control_flow.set_wait();
            match event {
                Event::Resumed => {
                    let window = self.window.take().unwrap_or_else(|| {
                        let window_builder = WindowBuilder::new();
                        glutin_winit::finalize_window(window_target, window_builder, &self.gl_config)
                            .unwrap()
                    });

                    let attrs = window.build_surface_attributes(<_>::default());
                    let gl_surface = unsafe {
                        self.gl_config.display().create_window_surface(&self.gl_config, &attrs).unwrap()
                    };

                    // Make it current.
                    let gl_context =
                        self.not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

                    // Try setting vsync.
                    /*if let Err(res) = gl_surface
                        .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                    {
                        eprintln!("Error setting vsync: {res:?}");
                    }*/

                    assert!(state.replace((gl_context, gl_surface, window)).is_none());

                    handler(AppEvent::Resume);
                },
                Event::Suspended => {
                    // This event is only raised on Android, where the backing NativeWindow for a GL
                    // Surface can appear and disappear at any moment.
                    println!("Android window removed");

                    // Destroy the GL Surface and un-current the GL Context before ndk-glue releases
                    // the window back to the system.
                    let (gl_context, ..) = state.take().unwrap();
                    assert!(self.not_current_gl_context
                        .replace(gl_context.make_not_current().unwrap())
                        .is_none());
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        println!("resize..");
                        if size.width != 0 && size.height != 0 {
                            // Some platforms like EGL require resizing GL surface to update the size
                            // Notable platforms here are Wayland and macOS, other don't require it
                            // and the function is no-op, but it's wise to resize it for portability
                            // reasons.
                            if let Some((gl_context, gl_surface, _)) = &state {
                                gl_surface.resize(
                                    gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );

                                unsafe {
                                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                                }
                            }
                        }
                    },
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    _ => (),
                },
                Event::RedrawRequested(_window_id)=>{
                    if let Some((gl_context, gl_surface, window)) = &state {
                        handler(AppEvent::Render);
                        //window.request_redraw();
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                },
                /*Event::RedrawEventsCleared => {
                    if let Some((gl_context, gl_surface, window)) = &state {
                        handler(AppEvent::Render);
                        window.request_redraw();
                        gl_surface.swap_buffers(gl_context).unwrap();
                    }
                },*/
                _ => (),
            }
        });
	}
}
