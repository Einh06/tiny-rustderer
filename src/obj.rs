#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 { 
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{ x, y, z }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub texcoord: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<(i64, i64, i64)>,
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

                let f1:Vec<i64> = comp[1].split('/').map(|s| s.parse::<i64>().unwrap() ).collect();
                let f2:Vec<i64> = comp[2].split('/').map(|s| s.parse::<i64>().unwrap() ).collect();
                let f3:Vec<i64> = comp[3].split('/').map(|s| s.parse::<i64>().unwrap() ).collect();

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

        Mesh {
            vertices,
            texcoord,
            normals,
            faces
        }
    }
}
