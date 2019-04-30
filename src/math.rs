use std::ops::{Add, Sub, Mul, Neg, Index, IndexMut};

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
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(self, v: Vec3f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
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

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, s: f32) -> Vec3f {
        Vec3f::new(self.x * s, self.y * s, self.z * s)
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
impl IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, c: usize) -> &mut f32 {
        match c {
        0 => &mut self.x,
        1 => &mut self.y,
        2 => &mut self.z,
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
        self.dot(self).sqrt()
    }

    pub fn dot(self, v: Vec4f) -> f32 {
        (self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w)
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

impl Neg for Vec4f {
    type Output = Vec4f;
    fn neg(self) -> Vec4f {
        Vec4f::new(-self.x, -self.y, -self.z, -self.w)
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

impl IndexMut<usize> for Vec4f {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
        0 => &mut self.x,
        1 => &mut self.y,
        2 => &mut self.z,
        3 => &mut self.w,
        _ => panic!("{} is out of bound for Vec4f", i),
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

    pub fn projection(coef: f32) -> Mat44 {
        let mut m = Mat44::identity();
        m.m[3][2] = coef;
        m
    }

    pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Mat44 {

        let w = (eye - center).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u).normalized();

        let mut res = Mat44::identity();

        res.m[0][0] = u.x; res.m[0][1] = u.y; res.m[0][2] = u.z;
        res.m[1][0] = v.x; res.m[1][1] = v.y; res.m[1][2] = v.z;
        res.m[2][0] = w.x; res.m[2][1] = w.y; res.m[2][2] = w.z;
        res.m[0][3] = -u.dot(eye); res.m[1][3] = -v.dot(eye); res.m[2][3] = -w.dot(eye);
        
        res
    }

    pub fn viewport(x: f32, y: f32, w: f32, h: f32, depth: f32) -> Mat44 {
        let mut m = Mat44::identity();

        m.m[0][3] = x+w/2.0;
        m.m[1][3] = y+h/2.0;
        m.m[2][3] = depth/255.0;

        m.m[0][0] = w/2.0;
        m.m[1][1] = h/2.0;
        m.m[2][2] = depth/255.0;

        m
    }

    pub fn transposed(&self) -> Mat44 {
        Mat44::new(self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0],
                   self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1],
                   self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2],
                   self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3])
    }

    pub fn inverse(&self) -> Mat44 {

        let m = &self.m;

        let f0 = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        let f1 = m[2][1] * m[3][3] - m[2][3] * m[3][1];
        let f2 = m[2][1] * m[3][2] - m[2][2] * m[3][1];
        let f3 = m[1][2] * m[3][3] - m[1][3] * m[3][2];
        let f4 = m[1][1] * m[3][3] - m[1][3] * m[3][1];
        let f5 = m[1][1] * m[3][2] - m[1][2] * m[3][1];
        let f6 = m[1][2] * m[2][3] - m[1][3] * m[2][2];
        let f7 = m[1][1] * m[2][3] - m[1][3] * m[2][1];
        let f8 = m[1][1] * m[2][2] - m[1][2] * m[2][1];
        let f9 = m[2][0] * m[3][3] - m[2][3] * m[3][0];
        let f10 = m[2][0] * m[3][2] - m[2][2] * m[3][0];
        let f11 = m[1][0] * m[3][3] - m[1][3] * m[3][0];
        let f12 = m[1][0] * m[3][2] - m[1][2] * m[3][0];
        let f13 = m[1][0] * m[2][3] - m[1][3] * m[2][0];
        let f14 = m[1][0] * m[2][2] - m[1][2] * m[2][0];
        let f15 = m[2][0] * m[3][1] - m[2][1] * m[3][0];
        let f16 = m[1][0] * m[3][1] - m[1][1] * m[3][0];
        let f17 = m[1][0] * m[2][1] - m[1][1] * m[2][0];

        let c0  =  m[1][1] * f0 - m[1][2] * f1  + m[1][3] * f2;
        let c1  = -m[0][1] * f0 + m[0][2] * f1  - m[0][3] * f2;
        let c2  =  m[0][1] * f3 - m[0][2] * f4  + m[0][3] * f5;
        let c3  = -m[0][1] * f6 + m[0][2] * f7  - m[0][3] * f8;
        let c4  = -m[1][0] * f0 + m[1][2] * f9  - m[1][3] * f10;
        let c5  =  m[0][0] * f0 - m[0][2] * f9  + m[0][3] * f10;
        let c6  = -m[0][0] * f3 + m[0][2] * f11 - m[0][3] * f12;
        let c7  =  m[0][0] * f6 - m[0][2] * f13 + m[0][3] * f14;
        let c8  =  m[1][0] * f1 - m[1][1] * f9  + m[1][3] * f15;
        let c9  = -m[0][0] * f1 + m[0][1] * f9  - m[0][3] * f15;
        let c10 =  m[0][0] * f4 - m[0][1] * f11 + m[0][3] * f16;
        let c11 = -m[0][0] * f7 + m[0][1] * f13 - m[0][3] * f17;
        let c12 = -m[1][0] * f2 + m[1][1] * f10 - m[1][2] * f15;
        let c13 =  m[0][0] * f2 - m[0][1] * f10 + m[0][2] * f15;
        let c14 = -m[0][0] * f5 + m[0][1] * f12 - m[0][2] * f16;
        let c15 =  m[0][0] * f8 - m[0][1] * f14 + m[0][2] * f17;


        let res = Mat44::new(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15);
        res * (m[0][0] * c0 + m[0][1] * c4 + m[0][2] * c8 + m[0][3] * c12)
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
        let m = &self.m;
        Mat44::new(m[0][0] * s,m[0][1] * s,m[0][2] * s, m[0][3] * s,
                   m[1][0] * s,m[1][1] * s,m[1][2] * s, m[1][3] * s,
                   m[2][0] * s,m[2][1] * s,m[2][2] * s, m[2][3] * s,
                   m[3][0] * s,m[3][1] * s,m[3][2] * s, m[3][3] * s)
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
