use std::ops::{Add, Sub, Mul, Index};

#[derive(Clone, Copy, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Vec2i{ 
        Vec2i {x, y}
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Vec2i;
    fn add(self, v: Vec2i) -> Vec2i {
        Vec2i::new( self.x + v.x, self.y + v.y )
    }
}

impl Sub<Vec2i> for Vec2i {
    type Output = Vec2i;
    fn sub(self, v: Vec2i) -> Vec2i {
        Vec2i::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f32> for Vec2i {
    type Output = Vec2i;
    fn mul(self, s: f32) -> Vec2i {
        Vec2i::new(((self.x as f32) * s) as i32, ((self.y as f32) * s) as i32)
    }
}

impl Index<usize> for Vec2i {
    type Output = i32;
    fn index(&self, c: usize) -> &i32 {
        match c {
        0 => &self.x,
        1 => &self.y,
        _ => panic!("{} is out of bound for Vec2i", c),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f{ 
        Vec2f {x, y}
    }
}

impl Add<Vec2f> for Vec2f {
    type Output = Vec2f;
    fn add(self, v: Vec2f) -> Vec2f {
        Vec2f::new( self.x + v.x, self.y + v.y )
    }
}

impl Sub<Vec2f> for Vec2f {
    type Output = Vec2f;
    fn sub(self, v: Vec2f) -> Vec2f {
        Vec2f::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    fn mul(self, s: f32) -> Vec2f {
        Vec2f::new(self.x * s, self.y * s)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x,y,z}
    }

    pub fn length(self) -> f32 {
        self.dot(self)
    }

    pub fn dot(self, v: Vec3f) -> f32 {
        (self.x * v.x + self.y * v.y + self.z * v.z).sqrt()
    }

    pub fn cross(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
    
    pub fn normalized(self) -> Vec3f { 
        let inv_len = 1_f32 / self.length();
        Vec3f::new(self.x * inv_len, self.y * inv_len, self.z * inv_len)
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;
    fn index(&self, c: usize) -> &f32 {
        match c {
        0 => &self.x,
        1 => &self.y,
        2 => &self.z,
        _ => panic!("{} is out of bound for Vec3f", c),
        }
    }
}

impl From<Vec4f> for Vec3f {
    fn from(v: Vec4f) -> Vec3f {
        let inv_w = 1.0 / v.w;
        Vec3f::new(v.x * inv_w, v.y * inv_w, v.z * inv_w)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f {x,y,z,w}
    }

    pub fn length(self) -> f32 {
        self.dot(self)
    }

    pub fn dot(self, v: Vec4f) -> f32 {
        (self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w).sqrt()
    }
    
    pub fn normalized(self) -> Vec4f { 
        let inv_len = 1_f32 / self.length();
        Vec4f::new(self.x * inv_len, self.y * inv_len, self.z * inv_len, self.w * inv_len)
    }
}

impl Sub<Vec4f> for Vec4f {
    type Output = Vec4f;
    fn sub(self, v: Vec4f) -> Vec4f {
        Vec4f::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w)
    }
}

impl Index<usize> for Vec4f {
    type Output = f32;
    fn index(&self, c: usize) -> &f32 {
        match c {
        0 => &self.x,
        1 => &self.y,
        2 => &self.z,
        3 => &self.w,
        _ => panic!("{} is out of bound for Vec4f", c),
        }
    }
}

impl From<Vec3f> for Vec4f {
    fn from(v: Vec3f) -> Self {
        Self::new(v.x, v.y, v.z, 1.0)
    }
}

#[derive(Copy, Clone)]
pub struct Mat44 {
    pub m: [[f32; 4]; 4]
}

impl Mat44 {
    pub fn new(m00: f32, m01: f32, m02: f32, m03: f32, 
           m10: f32, m11: f32, m12: f32, m13: f32, 
           m20: f32, m21: f32, m22: f32, m23: f32,  
           m30: f32, m31: f32, m32: f32, m33: f32 ) -> Mat44 {

        Mat44 {m: [[m00, m01, m02, m03], 
                   [m10, m11, m12, m13], 
                   [m20, m21, m22, m23], 
                   [m30, m31, m32, m33]] }

    }

    pub fn identity() -> Mat44 {
        Mat44 {m: [[1_f32, 0_f32, 0_f32, 0_f32], 
                   [0_f32, 1_f32, 0_f32, 0_f32], 
                   [0_f32, 0_f32, 1_f32, 0_f32], 
                   [0_f32, 0_f32, 0_f32, 1_f32]] }
    }

    pub fn scale(s: f32) -> Mat44 {
        Mat44::identity() * s
    }

    pub fn translation(dx: f32, dy: f32, dz: f32) -> Mat44 {
        Mat44 {m: [[0_f32, 0_f32, 0_f32, dx], 
                   [0_f32, 0_f32, 0_f32, dy], 
                   [0_f32, 0_f32, 0_f32, dz], 
                   [0_f32, 0_f32, 0_f32, 1_f32]] }
        
    }

    pub fn projection(camera: Vec3f) -> Mat44 {
        Mat44 {m: [[1_f32, 0_f32, 0_f32, 0_f32], 
                   [0_f32, 1_f32, 0_f32, 0_f32], 
                   [0_f32, 0_f32, 1_f32, 0_f32], 
                   [0_f32, 0_f32, -1_f32 / camera.z, 1_f32]] }

    }

    pub fn viewport(x: f32, y: f32, w: f32, h: f32, depth: f32) -> Mat44 {
        let mut m = Mat44::identity();
        m.m[0][3] = x+w/2.0;
        m.m[1][3] = y+h/2.0;
        m.m[2][3] = depth/2.0;

        m.m[0][0] = w/2.0;
        m.m[1][1] = h/2.0;
        m.m[2][2] = depth/2.0;
        m
    }
}

impl Default for Mat44 {
    fn default() -> Mat44 {
        Mat44::identity()
    }
}

impl Mul<f32> for Mat44 {
    type Output = Mat44;
    fn mul(self, s: f32) -> Mat44 {
        Mat44 {m: [[s,     0_f32, 0_f32, 0_f32], 
                   [0_f32, s,     0_f32, 0_f32], 
                   [0_f32, 0_f32, s,     0_f32], 
                   [0_f32, 0_f32, 0_f32, s]] }
    }
}

impl Mul<Vec4f> for Mat44 {
    type Output = Vec4f;
    fn mul(self, v: Vec4f) -> Vec4f {
        Vec4f::new(self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * v.w,
                   self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * v.w,
                   self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * v.w,
                   self.m[3][0] * v.x + self.m[3][1] * v.y + self.m[3][2] * v.z + self.m[3][3] * v.w)
    }
}

impl Mul<Mat44> for Mat44 {
    type Output = Mat44;

    fn mul(self, m: Mat44) -> Mat44 {
        let mut result = Mat44::identity();
        result.m[0][0] = self.m[0][0] * m.m[0][0] + self.m[0][1] * m.m[1][0] + self.m[0][2] * m.m[2][0] + self.m[0][3] * m.m[3][0];
        result.m[0][1] = self.m[0][0] * m.m[0][1] + self.m[0][1] * m.m[1][1] + self.m[0][2] * m.m[2][1] + self.m[0][3] * m.m[3][1];
        result.m[0][2] = self.m[0][0] * m.m[0][2] + self.m[0][1] * m.m[1][2] + self.m[0][2] * m.m[2][2] + self.m[0][3] * m.m[3][2];
        result.m[0][3] = self.m[0][0] * m.m[0][3] + self.m[0][1] * m.m[1][3] + self.m[0][2] * m.m[2][3] + self.m[0][3] * m.m[3][3];

        result.m[1][0] = self.m[1][0] * m.m[0][0] + self.m[1][1] * m.m[1][0] + self.m[1][2] * m.m[2][0] + self.m[1][3] * m.m[3][0];
        result.m[1][1] = self.m[1][0] * m.m[0][1] + self.m[1][1] * m.m[1][1] + self.m[1][2] * m.m[2][1] + self.m[1][3] * m.m[3][1];
        result.m[1][2] = self.m[1][0] * m.m[0][2] + self.m[1][1] * m.m[1][2] + self.m[1][2] * m.m[2][2] + self.m[1][3] * m.m[3][2];
        result.m[1][3] = self.m[1][0] * m.m[0][3] + self.m[1][1] * m.m[1][3] + self.m[1][2] * m.m[2][3] + self.m[1][3] * m.m[3][3];

        result.m[2][0] = self.m[2][0] * m.m[0][0] + self.m[2][1] * m.m[1][0] + self.m[2][2] * m.m[2][0] + self.m[2][3] * m.m[3][0];
        result.m[2][1] = self.m[2][0] * m.m[0][1] + self.m[2][1] * m.m[1][1] + self.m[2][2] * m.m[2][1] + self.m[2][3] * m.m[3][1];
        result.m[2][2] = self.m[2][0] * m.m[0][2] + self.m[2][1] * m.m[1][2] + self.m[2][2] * m.m[2][2] + self.m[2][3] * m.m[3][2];
        result.m[2][3] = self.m[2][0] * m.m[0][3] + self.m[2][1] * m.m[1][3] + self.m[2][2] * m.m[2][3] + self.m[2][3] * m.m[3][3];

        result.m[3][0] = self.m[3][0] * m.m[0][0] + self.m[3][1] * m.m[1][0] + self.m[3][2] * m.m[2][0] + self.m[3][3] * m.m[3][0];
        result.m[3][1] = self.m[3][0] * m.m[0][1] + self.m[3][1] * m.m[1][1] + self.m[3][2] * m.m[2][1] + self.m[3][3] * m.m[3][1];
        result.m[3][2] = self.m[3][0] * m.m[0][2] + self.m[3][1] * m.m[1][2] + self.m[3][2] * m.m[2][2] + self.m[3][3] * m.m[3][2];
        result.m[3][3] = self.m[3][0] * m.m[0][3] + self.m[3][1] * m.m[1][3] + self.m[3][2] * m.m[2][3] + self.m[3][3] * m.m[3][3];
        result
    }
}
