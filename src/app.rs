use glow::*;
use crate::render_context::RenderContext;
use crate::shader::Program;
use crate::mesh::Mesh;
use crate::obj_loader;
use crate::math::{Mat4, Vec3};
use crate::camera::Camera;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

pub struct App {
    render_context: RenderContext,
    program: Program,
    meshes: Vec<Mesh>,
    projection: Mat4,
    camera: Camera,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let render_context = RenderContext::new(event_loop);
        let gl = &render_context.gl;

        let shader_version = "#version 410";
        let vertex_shader_source = format!(
            "{}\n{}",
            shader_version,
            r#"
            layout (location = 0) in vec3 aPos;
            uniform mat4 uProjection;
            uniform mat4 uView;
            void main() {
                gl_Position = uProjection * uView * vec4(aPos, 1.0);
            }
            "#
        );
        let fragment_shader_source = format!(
            "{}\n{}",
            shader_version,
            r#"
            out vec4 FragColor;
            uniform vec4 uColor;
            void main() {
                FragColor = uColor;
            }
            "#
        );

        let program = Program::new(gl, &vertex_shader_source, &fragment_shader_source).expect("Failed to create program");
        
        let mesh_data = obj_loader::load_obj("stanford_bunny.obj").expect("Failed to load OBJ");
        let mesh = Mesh::new(gl, &mesh_data, [1.0, 0.5, 0.2, 1.0]).expect("Failed to create mesh");

        let size = render_context.window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        let projection = Mat4::perspective(45.0f32.to_radians(), aspect, 0.1, 100.0);

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.3),
            Vec3::new(0.0, 0.1, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        Self {
            render_context,
            program,
            meshes: vec![mesh],
            projection,
            camera,
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, window_target| {
            let gl = &self.render_context.gl;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        self.program.delete(gl);
                        for mesh in &self.meshes {
                            mesh.delete(gl);
                        }
                        window_target.exit();
                    }
                    WindowEvent::Resized(size) => {
                        self.render_context.resize(size.width, size.height);
                        let aspect = size.width as f32 / size.height as f32;
                        self.projection = Mat4::perspective(45.0f32.to_radians(), aspect, 0.1, 100.0);
                    }
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            gl.clear_color(0.1, 0.2, 0.3, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT);
                        }

                        self.program.bind(gl);
                        
                        if let Some(loc) = self.program.get_uniform_location(gl, "uProjection") {
                            self.program.set_uniform_mat4(gl, &loc, &self.projection);
                        }

                        if let Some(loc) = self.program.get_uniform_location(gl, "uView") {
                            let view = self.camera.get_view_matrix();
                            self.program.set_uniform_mat4(gl, &loc, &view);
                        }

                        for mesh in &self.meshes {
                            mesh.draw(gl, &self.program);
                        }
                        
                        self.render_context.swap_buffers();
                    }
                    _ => (),
                },
                Event::AboutToWait => {
                    self.render_context.window.request_redraw();
                }
                _ => (),
            }
        }).unwrap();
    }
}
