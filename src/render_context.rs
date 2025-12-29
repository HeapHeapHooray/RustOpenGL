use glow::*;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct RenderContext {
    pub window: Window,
    pub gl: Context,
    pub gl_surface: Surface<WindowSurface>,
    pub gl_context: PossiblyCurrentContext,
}

impl RenderContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("Rust OpenGL with Glow")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
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

        let window = window.unwrap();
        let raw_window_handle = window.raw_window_handle();

        let gl_display = gl_config.display();
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(800).unwrap(),
            NonZeroU32::new(600).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let s = std::ffi::CString::new(s).unwrap();
                gl_display.get_proc_address(&s) as *const _
            })
        };

        Self {
            window,
            gl,
            gl_surface,
            gl_context,
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.gl_surface.resize(
                &self.gl_context,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );
            unsafe {
                self.gl.viewport(0, 0, width as i32, height as i32);
            }
        }
    }

    pub fn swap_buffers(&self) {
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
}
