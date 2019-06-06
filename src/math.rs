use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub};

const EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Vec2i;
    fn add(self, v: Vec2i) -> Vec2i {
        Vec2i::new(self.x + v.x, self.y + v.y)
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
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }
}

impl Add<Vec2f> for Vec2f {
    type Output = Vec2f;
    fn add(self, v: Vec2f) -> Vec2f {
        Vec2f::new(self.x + v.x, self.y + v.y)
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
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline(always)]
    pub fn dot(self, v: Vec3f) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    #[inline(always)]
    pub fn cross(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    #[inline(always)]
    pub fn normalized(self) -> Vec3f {
        let inv_len = 1_f32 / self.length();
        Vec3f::new(self.x * inv_len, self.y * inv_len, self.z * inv_len)
    }

    pub fn xy(&self) -> Vec2f {
        Vec2f {x: self.x, y: self.y}
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign<Vec3f> for Vec3f {
    fn add_assign(&mut self, v: Vec3f) {
        *self = Vec3f::new(self.x + v.x, self.y + v.y, self.z + self.z);
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn sub(self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    #[inline(always)]
    fn neg(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    #[inline(always)]
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
        Vec3f::new(v.x, v.y, v.z)
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

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f { x, y, z, w }
    }

    #[inline(always)]
    pub fn from_vec3f(v: Vec3f, w: f32) -> Vec4f {
        Vec4f {x: v.x, y: v.y, z: v.z, w}
    }

    #[allow(dead_code)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[allow(dead_code)]
    pub fn dot(self, v: Vec4f) -> f32 {
        (self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w)
    }

    #[allow(dead_code)]
    pub fn normalized(self) -> Vec4f {
        let inv_len = 1_f32 / self.length();
        Vec4f::new(
            self.x * inv_len,
            self.y * inv_len,
            self.z * inv_len,
            self.w * inv_len,
        )
    }

    #[inline(always)]
    pub fn xyz(&self) -> Vec3f {
        Vec3f { x: self.x, y: self.y, z: self.z}
    }

    #[inline(always)]
    pub fn homogenize(self) -> Vec3f {
        let inv_w = 1.0 / self.w;
        Vec3f::new(self.x * inv_w, self.y * inv_w, self.z * inv_w)
    }
}

impl Add<Vec4f> for Vec4f {
    type Output = Vec4f;

    #[inline(always)]
    fn add(self, v: Vec4f) -> Vec4f {
        Vec4f::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w)
    }
}

impl Sub<Vec4f> for Vec4f {
    type Output = Vec4f;

    #[inline(always)]
    fn sub(self, v: Vec4f) -> Vec4f {
        Vec4f::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w)
    }
}

impl Neg for Vec4f {
    type Output = Vec4f;

    #[inline(always)]
    fn neg(self) -> Vec4f {
        Vec4f::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for Vec4f {
    type Output = Vec4f;

    #[inline(always)]
    fn mul(self, s: f32) -> Vec4f {
        Vec4f::new(self.x * s, self.y * s, self.z * s, self.w * s)
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
pub struct Mat33 {
    pub m: [[f32; 3]; 3],
}

impl Mat33 {
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Mat33 {
        Mat33 {
            m: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }

    pub fn from_row_vec(v1: Vec3f, v2: Vec3f, v3: Vec3f) -> Mat33 {
        Mat33 {
            m: [[v1.x, v1.y, v1.z], [v2.x, v2.y, v2.z], [v3.x, v3.y, v3.z]],
        }
    }

    pub fn from_col_vec(v1: Vec3f, v2: Vec3f, v3: Vec3f) -> Mat33 {
        Mat33 {
            m: [[v1.x, v2.x, v3.x], [v1.y, v2.y, v3.y], [v1.z, v2.z, v3.z]],
        }
    }

    pub fn transposed(&self) -> Mat33 {
        let m = &self.m;
        Mat33::new(
            m[0][0], m[1][0], m[2][0], m[0][1], m[1][1], m[2][1], m[0][2], m[1][2], m[2][2],
        )
    }

    pub fn determinant(&self) -> f32 {
        let m = &self.m;
        m[0][0] * m[1][1] * m[2][2] + m[0][1] * m[1][2] * m[2][0] + m[0][2] * m[1][0] * m[2][1]
            - m[0][2] * m[1][1] * m[2][0]
            - m[0][1] * m[1][0] * m[2][2]
            - m[0][0] * m[1][2] * m[2][1]
    }

    pub fn cofactor(&self) -> Mat33 {
        let m = &self.m;

        let mut res = Mat33::new(1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

        res.m[0][0] *= m[1][1] * m[2][2] - m[1][2] * m[2][1];
        res.m[0][1] *= m[1][0] * m[2][2] - m[1][2] * m[2][0];
        res.m[0][2] *= m[1][0] * m[2][1] - m[1][1] * m[2][0];

        res.m[1][0] *= m[0][1] * m[2][2] - m[0][2] * m[2][0];
        res.m[1][1] *= m[0][0] * m[2][2] - m[0][2] * m[2][0];
        res.m[1][2] *= m[0][0] * m[2][1] - m[0][1] * m[2][0];

        res.m[2][0] *= m[0][1] * m[1][2] - m[0][2] * m[1][1];
        res.m[2][1] *= m[0][0] * m[1][2] - m[0][2] * m[1][0];
        res.m[2][2] *= m[0][0] * m[1][1] - m[0][1] * m[1][0];

        res
    }

    pub fn inverse(&self) -> Mat33 {
        self.cofactor().transposed() * (1.0 / self.determinant())
    }
}

impl Mul<Vec3f> for Mat33 {
    type Output = Vec3f;

    fn mul(self, v: Vec3f) -> Vec3f {
        Vec3f::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }
}

impl Mul<f32> for Mat33 {
    type Output = Mat33;

    fn mul(self, s: f32) -> Mat33 {
        Mat33::new(
            self.m[0][0] * s,
            self.m[0][1] * s,
            self.m[0][2] * s,
            self.m[1][0] * s,
            self.m[1][1] * s,
            self.m[1][2] * s,
            self.m[2][0] * s,
            self.m[2][1] * s,
            self.m[2][2] * s,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Mat44 {
    pub m: [[f32; 4]; 4],
}

impl Mat44 {
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Mat44 {
        Mat44 {
            m: [
                [m00, m01, m02, m03],
                [m10, m11, m12, m13],
                [m20, m21, m22, m23],
                [m30, m31, m32, m33],
            ],
        }
    }

    pub fn identity() -> Mat44 {
        Mat44 {
            m: [
                [1_f32, 0_f32, 0_f32, 0_f32],
                [0_f32, 1_f32, 0_f32, 0_f32],
                [0_f32, 0_f32, 1_f32, 0_f32],
                [0_f32, 0_f32, 0_f32, 1_f32],
            ],
        }
    }

    #[allow(dead_code)]
    pub fn scale(s: f32) -> Mat44 {
        Mat44::identity() * s
    }

    #[allow(dead_code)]
    pub fn translation(dx: f32, dy: f32, dz: f32) -> Mat44 {
        Mat44 {
            m: [
                [0_f32, 0_f32, 0_f32, dx],
                [0_f32, 0_f32, 0_f32, dy],
                [0_f32, 0_f32, 0_f32, dz],
                [0_f32, 0_f32, 0_f32, 1_f32],
            ],
        }
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

        res.m[0][0] = u.x;
        res.m[0][1] = u.y;
        res.m[0][2] = u.z;
        res.m[0][3] = -center.x;
        res.m[1][0] = v.x;
        res.m[1][1] = v.y;
        res.m[1][2] = v.z;
        res.m[1][3] = -center.y;
        res.m[2][0] = w.x;
        res.m[2][1] = w.y;
        res.m[2][2] = w.z;
        res.m[2][3] = -center.z;

        res
    }

    pub fn viewport(x: f32, y: f32, w: f32, h: f32, depth: f32) -> Mat44 {
        let mut m = Mat44::identity();

        m.m[0][3] = x + w / 2.0;
        m.m[1][3] = y + h / 2.0;
        m.m[2][3] = depth / 2.0;

        m.m[0][0] = w / 2.0;
        m.m[1][1] = h / 2.0;
        m.m[2][2] = depth / 2.0;

        m
    }

    pub fn transposed(&self) -> Mat44 {
        Mat44::new(
            self.m[0][0],
            self.m[1][0],
            self.m[2][0],
            self.m[3][0],
            self.m[0][1],
            self.m[1][1],
            self.m[2][1],
            self.m[3][1],
            self.m[0][2],
            self.m[1][2],
            self.m[2][2],
            self.m[3][2],
            self.m[0][3],
            self.m[1][3],
            self.m[2][3],
            self.m[3][3],
        )
    }

    #[allow(dead_code)]
    pub fn trace(self) -> f32 {
        self.m[0][0] + self.m[1][1] + self.m[2][2] + self.m[3][3]
    }

    pub fn determinant(&self) -> f32 {
        let m = &self.m;

        let a = m[0][0]
            * (m[1][1] * m[2][2] * m[3][3]
                + m[1][2] * m[2][3] * m[3][1]
                + m[1][3] * m[2][1] * m[3][2]
                - m[1][3] * m[2][2] * m[3][1]
                - m[1][2] * m[2][1] * m[3][3]
                - m[1][1] * m[2][3] * m[3][2]);
        let b = m[0][1]
            * (m[1][0] * m[2][2] * m[3][3]
                + m[1][2] * m[2][3] * m[3][0]
                + m[1][3] * m[2][0] * m[3][2]
                - m[1][3] * m[2][2] * m[3][0]
                - m[1][2] * m[2][0] * m[3][3]
                - m[1][0] * m[2][3] * m[3][2]);
        let c = m[0][2]
            * (m[1][0] * m[2][1] * m[3][3]
                + m[1][1] * m[2][3] * m[3][0]
                + m[1][3] * m[2][0] * m[3][1]
                - m[1][3] * m[2][1] * m[3][0]
                - m[1][1] * m[2][0] * m[3][3]
                - m[1][0] * m[2][3] * m[3][1]);
        let d = m[0][3]
            * (m[1][0] * m[2][1] * m[3][2]
                + m[1][1] * m[2][2] * m[3][0]
                + m[1][2] * m[2][0] * m[3][1]
                - m[1][2] * m[2][1] * m[3][0]
                - m[1][1] * m[2][0] * m[3][2]
                - m[1][0] * m[2][2] * m[3][1]);

        a - b + c - d
    }

    pub fn cofactor(&self) -> Mat44 {
        let m = &self.m;
        let mut res = Mat44::new(
            1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0,
        );

        res.m[0][0] *=
            m[1][1] * m[2][2] * m[3][3] + m[1][2] * m[2][3] * m[3][1] + m[1][3] * m[2][1] * m[3][2]
                - m[1][3] * m[2][2] * m[3][1]
                - m[1][2] * m[2][1] * m[3][3]
                - m[1][1] * m[2][3] * m[3][2];
        res.m[0][1] *=
            m[1][0] * m[2][2] * m[3][3] + m[1][2] * m[2][3] * m[3][0] + m[1][3] * m[2][0] * m[3][2]
                - m[1][3] * m[2][2] * m[3][0]
                - m[1][2] * m[2][0] * m[3][3]
                - m[1][0] * m[2][3] * m[3][2];
        res.m[0][2] *=
            m[1][0] * m[2][1] * m[3][3] + m[1][1] * m[2][3] * m[3][0] + m[1][3] * m[2][0] * m[3][1]
                - m[1][3] * m[2][1] * m[3][0]
                - m[1][1] * m[2][0] * m[3][3]
                - m[1][0] * m[2][3] * m[3][1];
        res.m[0][3] *=
            m[1][0] * m[2][1] * m[3][2] + m[1][1] * m[2][2] * m[3][0] + m[1][2] * m[2][0] * m[3][1]
                - m[1][2] * m[2][1] * m[3][0]
                - m[1][1] * m[2][0] * m[3][2]
                - m[1][0] * m[2][2] * m[3][1];

        res.m[1][0] *=
            m[0][1] * m[2][2] * m[3][3] + m[0][2] * m[2][3] * m[3][1] + m[0][3] * m[2][1] * m[3][2]
                - m[0][3] * m[2][2] * m[3][1]
                - m[0][2] * m[2][1] * m[3][3]
                - m[0][1] * m[2][3] * m[3][2];
        res.m[1][1] *=
            m[0][0] * m[2][2] * m[3][3] + m[0][2] * m[2][3] * m[3][0] + m[0][3] * m[2][0] * m[3][2]
                - m[0][3] * m[2][2] * m[3][0]
                - m[0][2] * m[2][0] * m[3][3]
                - m[0][0] * m[2][3] * m[3][2];
        res.m[1][2] *=
            m[0][0] * m[2][1] * m[3][3] + m[0][1] * m[2][3] * m[3][0] + m[0][3] * m[2][0] * m[3][1]
                - m[0][3] * m[2][1] * m[3][0]
                - m[0][1] * m[2][0] * m[3][3]
                - m[0][0] * m[2][3] * m[3][1];
        res.m[1][3] *=
            m[0][0] * m[2][1] * m[3][2] + m[0][1] * m[2][2] * m[3][0] + m[0][2] * m[2][0] * m[3][1]
                - m[0][2] * m[2][1] * m[3][0]
                - m[0][1] * m[2][0] * m[3][2]
                - m[0][0] * m[2][2] * m[3][1];

        res.m[2][0] *=
            m[0][1] * m[1][2] * m[3][3] + m[0][2] * m[1][3] * m[3][1] + m[0][3] * m[1][1] * m[3][2]
                - m[0][3] * m[1][2] * m[3][1]
                - m[0][2] * m[1][1] * m[3][3]
                - m[0][1] * m[1][3] * m[3][2];
        res.m[2][1] *=
            m[0][0] * m[1][2] * m[3][3] + m[0][2] * m[1][3] * m[3][0] + m[0][3] * m[1][0] * m[3][2]
                - m[0][3] * m[1][2] * m[3][0]
                - m[0][2] * m[1][0] * m[3][3]
                - m[0][0] * m[1][3] * m[3][2];
        res.m[2][2] *=
            m[0][0] * m[1][1] * m[3][3] + m[0][1] * m[1][3] * m[3][0] + m[0][3] * m[1][0] * m[3][1]
                - m[0][3] * m[1][1] * m[3][0]
                - m[0][1] * m[1][0] * m[3][3]
                - m[0][0] * m[1][3] * m[3][1];
        res.m[2][3] *=
            m[0][0] * m[1][1] * m[3][2] + m[0][1] * m[1][2] * m[3][0] + m[0][2] * m[1][0] * m[3][1]
                - m[0][2] * m[1][1] * m[3][0]
                - m[0][1] * m[1][0] * m[3][2]
                - m[0][0] * m[1][2] * m[3][1];

        res.m[3][0] *=
            m[0][1] * m[1][2] * m[2][3] + m[0][2] * m[1][3] * m[2][1] + m[0][3] * m[1][1] * m[2][2]
                - m[0][3] * m[1][2] * m[2][1]
                - m[0][2] * m[1][1] * m[2][3]
                - m[0][1] * m[1][3] * m[2][2];
        res.m[3][1] *=
            m[0][0] * m[1][2] * m[2][3] + m[0][2] * m[1][3] * m[2][0] + m[0][3] * m[1][0] * m[2][2]
                - m[0][3] * m[1][2] * m[2][0]
                - m[0][2] * m[1][0] * m[2][3]
                - m[0][0] * m[1][3] * m[2][2];
        res.m[3][2] *=
            m[0][0] * m[1][1] * m[2][3] + m[0][1] * m[1][3] * m[2][0] + m[0][3] * m[1][0] * m[2][1]
                - m[0][3] * m[1][1] * m[2][0]
                - m[0][1] * m[1][0] * m[2][3]
                - m[0][0] * m[1][3] * m[2][1];
        res.m[3][3] *=
            m[0][0] * m[1][1] * m[2][2] + m[0][1] * m[1][2] * m[2][0] + m[0][2] * m[1][0] * m[2][1]
                - m[0][2] * m[1][1] * m[2][0]
                - m[0][1] * m[1][0] * m[2][2]
                - m[0][0] * m[1][2] * m[2][1];

        res
    }

    pub fn inverse(&self) -> Mat44 {
        let det = self.determinant();
        // TODO: Change that to return result
        if det.abs() < EPSILON {
            panic!("Bad determinant")
        }

        self.cofactor().transposed() * (1.0 / det)
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
        Mat44::new(
            m[0][0] * s,
            m[0][1] * s,
            m[0][2] * s,
            m[0][3] * s,
            m[1][0] * s,
            m[1][1] * s,
            m[1][2] * s,
            m[1][3] * s,
            m[2][0] * s,
            m[2][1] * s,
            m[2][2] * s,
            m[2][3] * s,
            m[3][0] * s,
            m[3][1] * s,
            m[3][2] * s,
            m[3][3] * s,
        )
    }
}

impl Mul<Vec4f> for Mat44 {
    type Output = Vec4f;
    fn mul(self, v: Vec4f) -> Vec4f {
        Vec4f::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * v.w,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * v.w,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * v.w,
            self.m[3][0] * v.x + self.m[3][1] * v.y + self.m[3][2] * v.z + self.m[3][3] * v.w,
        )
    }
}

impl Mul<Mat44> for Mat44 {
    type Output = Mat44;

    fn mul(self, m: Mat44) -> Mat44 {
        let mut result = Mat44::identity();
        result.m[0][0] = self.m[0][0] * m.m[0][0]
            + self.m[0][1] * m.m[1][0]
            + self.m[0][2] * m.m[2][0]
            + self.m[0][3] * m.m[3][0];
        result.m[0][1] = self.m[0][0] * m.m[0][1]
            + self.m[0][1] * m.m[1][1]
            + self.m[0][2] * m.m[2][1]
            + self.m[0][3] * m.m[3][1];
        result.m[0][2] = self.m[0][0] * m.m[0][2]
            + self.m[0][1] * m.m[1][2]
            + self.m[0][2] * m.m[2][2]
            + self.m[0][3] * m.m[3][2];
        result.m[0][3] = self.m[0][0] * m.m[0][3]
            + self.m[0][1] * m.m[1][3]
            + self.m[0][2] * m.m[2][3]
            + self.m[0][3] * m.m[3][3];

        result.m[1][0] = self.m[1][0] * m.m[0][0]
            + self.m[1][1] * m.m[1][0]
            + self.m[1][2] * m.m[2][0]
            + self.m[1][3] * m.m[3][0];
        result.m[1][1] = self.m[1][0] * m.m[0][1]
            + self.m[1][1] * m.m[1][1]
            + self.m[1][2] * m.m[2][1]
            + self.m[1][3] * m.m[3][1];
        result.m[1][2] = self.m[1][0] * m.m[0][2]
            + self.m[1][1] * m.m[1][2]
            + self.m[1][2] * m.m[2][2]
            + self.m[1][3] * m.m[3][2];
        result.m[1][3] = self.m[1][0] * m.m[0][3]
            + self.m[1][1] * m.m[1][3]
            + self.m[1][2] * m.m[2][3]
            + self.m[1][3] * m.m[3][3];

        result.m[2][0] = self.m[2][0] * m.m[0][0]
            + self.m[2][1] * m.m[1][0]
            + self.m[2][2] * m.m[2][0]
            + self.m[2][3] * m.m[3][0];
        result.m[2][1] = self.m[2][0] * m.m[0][1]
            + self.m[2][1] * m.m[1][1]
            + self.m[2][2] * m.m[2][1]
            + self.m[2][3] * m.m[3][1];
        result.m[2][2] = self.m[2][0] * m.m[0][2]
            + self.m[2][1] * m.m[1][2]
            + self.m[2][2] * m.m[2][2]
            + self.m[2][3] * m.m[3][2];
        result.m[2][3] = self.m[2][0] * m.m[0][3]
            + self.m[2][1] * m.m[1][3]
            + self.m[2][2] * m.m[2][3]
            + self.m[2][3] * m.m[3][3];

        result.m[3][0] = self.m[3][0] * m.m[0][0]
            + self.m[3][1] * m.m[1][0]
            + self.m[3][2] * m.m[2][0]
            + self.m[3][3] * m.m[3][0];
        result.m[3][1] = self.m[3][0] * m.m[0][1]
            + self.m[3][1] * m.m[1][1]
            + self.m[3][2] * m.m[2][1]
            + self.m[3][3] * m.m[3][1];
        result.m[3][2] = self.m[3][0] * m.m[0][2]
            + self.m[3][1] * m.m[1][2]
            + self.m[3][2] * m.m[2][2]
            + self.m[3][3] * m.m[3][2];
        result.m[3][3] = self.m[3][0] * m.m[0][3]
            + self.m[3][1] * m.m[1][3]
            + self.m[3][2] * m.m[2][3]
            + self.m[3][3] * m.m[3][3];
        result
    }
}

impl std::fmt::Debug for Mat44 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Mat44 {{\n {:?}\n {:?}\n {:?} \n {:?}\n}}\n",
            self.m[0], self.m[1], self.m[2], self.m[3]
        )
    }
}

impl std::fmt::Debug for Mat33 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Mat33 {{\n {:?}\n {:?}\n {:?}\n\n",
            self.m[0], self.m[1], self.m[2]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Mat44 {
        fn eq(&self, rhs: &Mat44) -> bool {
            for i in 0..4 {
                for j in 0..4 {
                    if (self.m[i][j] - rhs.m[i][j]) > EPSILON {
                        return false;
                    }
                }
            }

            true
        }
    }

    impl PartialEq for Mat33 {
        fn eq(&self, rhs: &Mat33) -> bool {
            for i in 0..3 {
                for j in 0..3 {
                    if (self.m[i][j] - rhs.m[i][j]) > EPSILON {
                        return false;
                    }
                }
            }

            true
        }
    }

    #[test]
    fn mat33_determinant_test() {
        let m = Mat33::new(
            4.000, 3.000, 8.000, 2.000, 5.000, 7.000, 8.000, 1.000, 6.000,
        );
        assert_eq!(m.determinant(), -80.000);

        let m = Mat33::new(
            1.000, 9.000, 9.000, 8.000, 0.000, 7.000, 4.000, 2.000, 3.000,
        );
        assert_eq!(m.determinant(), 166.000);
    }

    #[test]
    fn mat33_inverse_test() {
        let m = Mat33::new(
            4.000, 3.000, 8.000, 2.000, 5.000, 7.000, 8.000, 1.000, 6.000,
        );
        let expected = Mat33::new(
            -0.288, 0.125, 0.238, -0.550, 0.500, 0.150, 0.475, -0.250, -0.175,
        );

        assert_eq!(m.inverse(), expected);
    }

    #[test]
    fn mat44_determinant_test() {
        let m = Mat44::identity();
        assert_eq!(m.determinant(), 1.0);

        let m = Mat44::new(
            1.0, 3.0, 5.0, 9.0, 1.0, 3.0, 1.0, 7.0, 4.0, 3.0, 9.0, 7.0, 5.0, 2.0, 0.0, 9.0,
        );
        assert_eq!(m.determinant(), -376.0);

        let m = Mat44::new(
            5.0, 0.0, 8.0, 7.0, 6.0, 5.0, 8.0, 5.0, 9.0, 1.0, 0.0, 7.0, 6.0, 2.0, 1.0, 8.0,
        );
        assert_eq!(m.determinant(), -1317.0);

        let m = Mat44::new(
            5.0, 2.0, 4.0, 9.0, 6.0, 7.0, 7.0, 9.0, 4.0, 1.0, 9.0, 6.0, 4.0, 2.0, 8.0, 7.0,
        );
        assert!((m.determinant() - (146.0)) < EPSILON);

        let m = Mat44::new(
            5.2, 5.8, 5.5, 7.7, 1.9, 4.5, 3.5, 6.3, 1.3, 6.1, 3.3, 4.4, 5.1, 4.6, 3.5, 1.1,
        );
        assert!((m.determinant() - (-5.790)) < EPSILON);
    }

    #[test]
    fn mat44_inverse_test() {
        let m = Mat44::new(
            5.2, 5.8, 5.5, 7.7, 1.9, 4.5, 3.5, 6.3, 1.3, 6.1, 3.3, 4.4, 5.1, 4.6, 3.5, 1.1,
        );
        let res = m.inverse();

        let expected = Mat44::new(
            -5.74426, 9.26688, -4.12932, 3.65318, -4.52343, 6.94351, -2.70452, 2.71471, 15.45360,
            -24.52360, 10.38651, -9.26791, -3.62192, 6.02853, -2.59313, 2.10801,
        );

        assert_eq!(res, expected);
    }
}
