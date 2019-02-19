use std::fs::{DirBuilder, File};use std::io::{Write, Read};

mod ppm;

pub mod obj {
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
}

fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut ppm::Image, color: ppm::RGB) {
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    let (x0, x1, y0, y1) = if steep {(y0, y1, x0, x1) } else { (x0, x1, y0, y1) }; // SWAP
    let (x0, x1, y0, y1) = if x0>x1 {(x1, x0, y1, y0) } else { (x0, x1, y0, y1) }; // SWAP

    let (dx, dy) = (x1-x0, y1-y0);
    let (derror, mut error) = (dy.abs() * 2, 0);

    let mut y = y0;
    let yinc = if y1 > y0 { 1 } else { -1 };

    for x in x0..x1 {
        if steep { 
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
        
        error += derror;
        if error > dx {
            y += yinc;
            error -= dx * 2;
        }
    }
}

fn main() -> std::io::Result<()> {

    let width = 800;
    let height = 800;
    let fwidth = (width - 1) as f32;
    let fheight = (height - 1) as f32;

    let mut image = ppm::Image::new(width, height);

    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");
    resource_dir.push("african_head.obj");

    println!("Opening model file");
    let mut file = File::open(resource_dir.as_path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("loading mesh content");
    let mesh = obj::Mesh::load(&content[..]);

    for chunk in mesh.faces.chunks(3) {
        let (i1, _, _) = chunk[0];
        let (i2, _, _) = chunk[1];
        let (i3, _, _) = chunk[2];

        let v1: obj::Vec3 = mesh.vertices[i1 as usize];
        let v2: obj::Vec3 = mesh.vertices[i2 as usize];
        let v3: obj::Vec3 = mesh.vertices[i3 as usize];

        let v1 = obj::Vec3::new( (v1.x + 1_f32) / 2_f32, (v1.y + 1_f32) / 2_f32, (v1.z + 1_f32) / 2_f32 );
        let v2 = obj::Vec3::new( (v2.x + 1_f32) / 2_f32, (v2.y + 1_f32) / 2_f32 ,(v1.z + 1_f32) / 2_f32 );
        let v3 = obj::Vec3::new( (v3.x + 1_f32) / 2_f32, (v3.y + 1_f32) / 2_f32 ,(v1.z + 1_f32) / 2_f32 );
        
        let (x1, y1) = ( (fwidth * v1.x) as i32, (fheight * v1.y) as i32);
        let (x2, y2) = ( (fwidth * v2.x) as i32, (fheight * v2.y) as i32);
        let (x3, y3) = ( (fwidth * v3.x) as i32, (fheight * v3.y) as i32);

        line(x1, y1, x2, y2, &mut image, ppm::RGB::white());
        line(x2, y2, x3, y3, &mut image, ppm::RGB::white());
        line(x3, y3, x1, y1, &mut image, ppm::RGB::white());
    }

    println!("opening the output");
    let mut output_dir = std::env::current_dir().unwrap();
    output_dir.push("output");

    DirBuilder::new()
        .recursive(true)
        .create(output_dir.as_path())
        .unwrap();

    output_dir.push("result.ppm");
    let mut file = File::create(output_dir.as_path())?;

    println!("Writing to output");
    file.write(String::from(&image).as_bytes())?;

    println!("Done!");
    Ok(())
}
