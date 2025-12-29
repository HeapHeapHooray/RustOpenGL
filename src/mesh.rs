use glow::*;
use crate::vertex_buffer::VertexBuffer;
use crate::vertex_array::VertexArray;
use crate::index_buffer::IndexBuffer;
use crate::mesh_data::MeshData;
use crate::shader::Program;

pub struct Mesh {
    vao: VertexArray,
    vbo: VertexBuffer,
    ebo: IndexBuffer,
    pub color: [f32; 4],
}

impl Mesh {
    pub fn new(gl: &Context, mesh_data: &MeshData, color: [f32; 4]) -> Result<Self, String> {
        let vao = VertexArray::new(gl)?;
        let vbo = VertexBuffer::new(gl)?;
        let ebo = IndexBuffer::new(gl, &mesh_data.indices, glow::STATIC_DRAW)?;

        vao.bind(gl);
        vbo.set_data(gl, &mesh_data.vertices, glow::STATIC_DRAW);
        ebo.bind(gl);

        vao.set_attribute_f32(gl, 0, 3, 3 * core::mem::size_of::<f32>() as i32, 0);

        vbo.unbind(gl);
        // Note: Do NOT unbind EBO while VAO is bound, as it's part of VAO state.
        vao.unbind(gl);
        ebo.unbind(gl);

        Ok(Self { vao, vbo, ebo, color })
    }

    pub fn draw(&self, gl: &Context, program: &Program) {
        if let Some(location) = program.get_uniform_location(gl, "uColor") {
            program.set_uniform_4f(gl, &location, self.color[0], self.color[1], self.color[2], self.color[3]);
        }

        self.vao.bind(gl);
        unsafe {
            gl.draw_elements(glow::TRIANGLES, self.ebo.count(), glow::UNSIGNED_INT, 0);
        }
    }

    pub fn delete(&self, gl: &Context) {
        self.vao.delete(gl);
        self.vbo.delete(gl);
        self.ebo.delete(gl);
    }
}
