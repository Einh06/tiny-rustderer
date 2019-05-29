use math::{Vec2f, Vec3f};

pub struct Triangle {
    pub t: [(usize, usize, usize); 3],
}

impl Triangle {
    fn new(
        t1: (usize, usize, usize),
        t2: (usize, usize, usize),
        t3: (usize, usize, usize),
    ) -> Triangle {
        Triangle { t: [t1, t2, t3] }
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = (usize, usize, usize);
    fn index(&self, i: usize) -> &(usize, usize, usize) {
        &self.t[i]
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub texcoord: Vec<Vec2f>,
    pub normals: Vec<Vec3f>,
    pub tangents: Vec<Vec3f>,
    pub faces: Vec<Triangle>,
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
                        comp[3].parse::<f32>().unwrap(),
                    );
                    vertices.push(Vec3f { x, y, z })
                }
                "vt" => {
                    let (x, y) = (
                        comp[2].parse::<f32>().expect("Can't parse texcoord float"),
                        comp[3].parse::<f32>().expect("Can't parse texcoord float"),
                    );
                    texcoord.push(Vec2f { x, y })
                }
                "vn" => {
                    let (x, y, z) = (
                        comp[2].parse::<f32>().unwrap(),
                        comp[3].parse::<f32>().unwrap(),
                        comp[4].parse::<f32>().unwrap(),
                    );
                    normals.push(Vec3f { x, y, z })
                }
                "f" => {
                    let parse_obj_indices = |s: &str| s.parse::<usize>().unwrap() - 1;

                    let f1: Vec<usize> = comp[1].split('/').map(parse_obj_indices).collect();
                    let f2: Vec<usize> = comp[2].split('/').map(parse_obj_indices).collect();
                    let f3: Vec<usize> = comp[3].split('/').map(parse_obj_indices).collect();

                    let t = Triangle::new(
                        (f1[0], f1[1], f1[2]),
                        (f2[0], f2[1], f2[2]),
                        (f3[0], f3[1], f3[2]),
                    );

                    faces.push(t);
                }
                _ => continue,
            }
        }

        vertices.shrink_to_fit();
        texcoord.shrink_to_fit();
        normals.shrink_to_fit();
        faces.shrink_to_fit();

        // Assumes that obj file uses faces
        // Could be done in 1 pass
        let tangents = {
            let mut res = Vec::with_capacity(vertices.len());
            for _ in 0..vertices.len() {
                res.push(Vec3f::new(0.0, 0.0, 0.0))
            }
            let mut tan: Vec<Vec3f> = Vec::with_capacity(vertices.len());
            for _ in 0..vertices.len() {
                tan.push(Vec3f::new(0.0, 0.0, 0.0))
            }

            for triangle in &faces {
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
            res.shrink_to_fit();

            res
        };

        Mesh {
            vertices,
            texcoord,
            normals,
            tangents,
            faces,
        }
    }
}
