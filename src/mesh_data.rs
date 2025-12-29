pub struct MeshData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn from_data(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(x);
        self.vertices.push(y);
        self.vertices.push(z);
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }
}
