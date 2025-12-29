mod shader;
mod render_context;
mod mesh;
mod app;
mod vertex_buffer;
mod vertex_array;
mod mesh_data;
mod obj_loader;
mod index_buffer;
mod math;
mod camera;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let app = App::new(&event_loop);
    app.run(event_loop);
}
