#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len == 0.0 {
            return self;
        }
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        let mut data = [0.0; 16];
        
        data[0] = f / aspect;
        data[5] = f;
        data[10] = (far + near) / (near - far);
        data[11] = -1.0;
        data[14] = (2.0 * far * near) / (near - far);
        
        Self { data }
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = center.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        let mut data = [0.0; 16];
        data[0] = s.x;
        data[4] = s.y;
        data[8] = s.z;

        data[1] = u.x;
        data[5] = u.y;
        data[9] = u.z;

        data[2] = -f.x;
        data[6] = -f.y;
        data[10] = -f.z;

        data[12] = -s.dot(eye);
        data[13] = -u.dot(eye);
        data[14] = f.dot(eye);
        data[15] = 1.0;

        Self { data }
    }

    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let mut data = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[i + j * 4] += self.data[i + k * 4] * other.data[k + j * 4];
                }
            }
        }
        Mat4 { data }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
}
