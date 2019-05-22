#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 { 
    #[allow(dead_code)]
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{ x, y, z }
    }

    fn dot(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalized(self) -> Vec3{
        let inv_len = 1.0 / self.length();
        Vec3::new(self.x *inv_len, self.y * inv_len, self.z * inv_len)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub texcoord: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub tangents: Vec<Vec3>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn load(content: &str) -> Mesh {
        let mut vertices = Vec::new();
        let mut texcoord = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();

        for line in content.lines() {
            let comp: Vec<&str> = line.split(' ').collect();

            match comp[0] {
            "v" => {
                let (x, y, z) = (
                    comp[1].parse::<f32>().unwrap(),
                    comp[2].parse::<f32>().unwrap(),
                    comp[3].parse::<f32>().unwrap());
                vertices.push(Vec3{x,y,z})
            },
            "vt" => { 
                let (x, y, z) = (
                    comp[2].parse::<f32>().expect("Can't parse texcoord float"),
                    comp[3].parse::<f32>().expect("Can't parse texcoord float"),
                    comp[4].parse::<f32>().expect("Can't parse texcoord float"));
                texcoord.push(Vec3{x,y,z}) 
            },
            "vn" => {
                let (x, y, z) = (
                    comp[2].parse::<f32>().unwrap(),
                    comp[3].parse::<f32>().unwrap(),
                    comp[4].parse::<f32>().unwrap());
                normals.push(Vec3{x,y,z})
            },
            "f" => {

                let f1:Vec<usize> = comp[1].split('/').map(|s| s.parse::<usize>().unwrap() ).collect();
                let f2:Vec<usize> = comp[2].split('/').map(|s| s.parse::<usize>().unwrap() ).collect();
                let f3:Vec<usize> = comp[3].split('/').map(|s| s.parse::<usize>().unwrap() ).collect();

                faces.push((f1[0] - 1, f1[1] - 1, f1[2] - 1));
                faces.push((f2[0] - 1, f2[1] - 1, f2[2] - 1));
                faces.push((f3[0] - 1, f3[1] - 1, f3[2] - 1));
            },
            _ => continue,
            }
        }

        vertices.shrink_to_fit();
        texcoord.shrink_to_fit();
        normals.shrink_to_fit();
        faces.shrink_to_fit();

        let tangents = {

            let mut res = Vec::with_capacity(vertices.len());
            for _ in 0..vertices.len() { res.push(Vec3::new(0.0, 0.0, 0.0)) }
            let mut tan: Vec<Vec3> = Vec::with_capacity(vertices.len());
            for _ in 0..vertices.len() { tan.push(Vec3::new(0.0, 0.0, 0.0)) }

            for triangle in faces.chunks(3) {
                let (v1_index, t1_index, _) = triangle[0];
                let (v2_index, t2_index, _) = triangle[1];
                let (v3_index, t3_index, _) = triangle[2];

                let v1 = vertices[v1_index];
                let v2 = vertices[v2_index];
                let v3 = vertices[v3_index];

                let uv1 = texcoord[t1_index];
                let uv2 = texcoord[t2_index];
                let uv3 = texcoord[t3_index];

                let edge1 = v2 - v1;
                let edge2 = v3 - v1;

                let detla_uv1 = uv2 - uv1;
                let delta_uv2 = uv3 - uv1;
                
                let r = 1.0 / (detla_uv1.x * delta_uv2.y - delta_uv2.x * detla_uv1.y);
                let tangent = (edge1 * delta_uv2.y - edge2 * detla_uv1.y) * r;

                tan[v1_index] += tangent;
                tan[v2_index] += tangent;
                tan[v3_index] += tangent;

                // @Incomplete: We will need the bitangent if we start dealing with mirrored
            }

            for i in 0..vertices.len() {
                let n = normals[i];
                let t = tan[i];

                res[i] = (t - n * n.dot(t)).normalized(); // orthogonalization
            }

            res
        };

        Mesh {
            vertices,
            texcoord,
            normals,
            tangents,
            faces
        }
    }
}
