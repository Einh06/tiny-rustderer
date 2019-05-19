extern crate stb_image;
mod math;
mod ppm;
mod obj;

use std::fs::{DirBuilder, File};
use std::io::{Write, Read};
use stb_image::image;
use math::{Vec3f, Vec4f, Mat44};

use image::LoadResult::{Error, ImageU8, ImageF32};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;

impl From<obj::Vec3> for Vec3f {
    fn from(v: obj::Vec3) -> Vec3f {
        Vec3f::new(v.x, v.y, v.z)
    }
}

#[allow(dead_code)]
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

fn barycenter(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec3f) -> Vec3f {
    let v1 = Vec3f { 
        x: c.x - a.x,
        y: b.x - a.x,
        z: a.x - p.x,
    };

    let v2 = Vec3f {
        x: c.y - a.y,
        y: b.y - a.y,
        z: a.y - p.y,
    };

    let u = v1.cross(v2);
    if u.z.abs() < 1_f32 { Vec3f::new(-1_f32, 1_f32, 1_f32) }
    else { Vec3f::new(1_f32 - (u.x+u.y) / u.z, u.y / u.z, u.x / u.z) }
}

fn render_mesh(filename: &str, texture_name: &str, normal_map_name: &str, image: &mut ppm::Image, z_buffer: &mut [f32]) -> std::io::Result<()> {
    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");
    let mut texture_path = resource_dir.clone();
    let mut normal_map_path = resource_dir.clone();

    resource_dir.push(filename);
    texture_path.push(texture_name);
    normal_map_path.push(normal_map_name);

    let texture: image::Image<u8>;
    match image::load(texture_path.as_path()) {
        Error(str) => panic!(str),
        ImageU8(image) => texture = image,
        ImageF32(_image) => panic!("Wrong image format"),
    };

    let normal_map: image::Image<u8>;
    match image::load(texture_path.as_path()) {
        Error(str) => panic!(str),
        ImageU8(image) => normal_map = image,
        ImageF32(_image) => panic!("Wrong image format"),
    };

    println!("Opening model file");
    let mut file = File::open(resource_dir.as_path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("loading mesh content");
    let mesh = obj::Mesh::load(&content[..]);

    let eye = Vec3f::new(1.0, 1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);
    let up = Vec3f::new(0.0, 1.0, 0.0);

    let eye_from_object = Mat44::lookat(eye, center, up);
    let view_from_eye = Mat44::projection(-1.0/(eye - center).length());
    let screen_from_view = Mat44::viewport(FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, 255.0);
    let view_from_object = view_from_eye * eye_from_object;
    let screen_from_object = screen_from_view * view_from_object;

    let eye_from_object_it = eye_from_object.inverse().transposed();

    let light_dir = Vec4f::new(1.0, 1.0, 1.0, 0.0);
    let trans_light_dir = Vec3f::from(view_from_object * light_dir).normalized();

    for chunk in mesh.faces.chunks(3) {
        let (i1, t1, _n1) = chunk[0];
        let (i2, t2, _n2) = chunk[1];
        let (i3, t3, _n3) = chunk[2];

        let sv1 = (screen_from_object * Vec4f::from(Vec3f::from(mesh.vertices[i1]))).homogenize();
        let sv2 = (screen_from_object * Vec4f::from(Vec3f::from(mesh.vertices[i2]))).homogenize();
        let sv3 = (screen_from_object * Vec4f::from(Vec3f::from(mesh.vertices[i3]))).homogenize();

        let uv1 = Vec3f::from(mesh.texcoord[t1]);
        let uv2 = Vec3f::from(mesh.texcoord[t2]);
        let uv3 = Vec3f::from(mesh.texcoord[t3]);

        // bounding box for triangle
        let ixmin = sv1.x.min(sv2.x.min(sv3.x)).max(0.0) as i32;
        let iymin = sv1.y.min(sv2.y.min(sv3.y)).max(0.0) as i32;
        let ixmax = sv1.x.max(sv2.x.max(sv3.x)).min(FWIDTH - 1.0) as i32;
        let iymax = sv1.y.max(sv2.y.max(sv3.y)).min(FHEIGHT - 1.0) as i32;

        for y in iymin..=iymax {
            for x in ixmin..=ixmax {
                let xu = x as usize;
                let yu = y as usize;

                let mut p = Vec3f::new(x as f32, y as f32, 0.0);

                let bar = barycenter(sv1, sv2, sv3, p);
                if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 { continue }

                p.z = sv1.z * bar.x + sv2.z * bar.y + sv3.z * bar.z;
                let mut zb = &mut z_buffer[(yu*WIDTH)+xu];

                if *zb < p.z { 
                    *zb = p.z;
                    let u = bar.x * uv1.x + bar.y * uv2.x + bar.z * uv3.x;
                    let v = bar.x * uv1.y + bar.y * uv2.y + bar.z * uv3.y;

                    // get normal from normal map
                    let fnwidth  = normal_map.width  as f32;
                    let fnheight = normal_map.height as f32;
                    let nu = (u * fnwidth) as usize;
                    let nv = ((1.0 - v) * (fnheight - 1.0)) as usize; //flipped vertically

                    let pixel_index = (nv * normal_map.width + nu) * 3;
                    let nx = f32::from(normal_map.data[pixel_index + 0]) / 255.0; 
                    let ny = f32::from(normal_map.data[pixel_index + 1]) / 255.0; 
                    let nz = f32::from(normal_map.data[pixel_index + 2]) / 255.0; 

                    let normal = Vec3f::from(eye_from_object_it * Vec4f::new(nx, ny, nz, 0.0)).normalized();
                    let intensity = normal.dot(trans_light_dir).max(0.0);

                    // get texture pixel color from texture
                    let ftwidth = texture.width as f32;
                    let ftheight = texture.height as f32;

                    let tu = (u * ftwidth) as usize;
                    let tv = ((1.0 - v) * (ftheight - 1.0)) as usize; //flipped vertically

                    let pixel_index = (tv * texture.width + tu) * 3;
                    let r = texture.data[pixel_index + 0]; // RGB
                    let g = texture.data[pixel_index + 1]; // RGB
                    let b = texture.data[pixel_index + 2]; // RGB

                    let r = (f32::from(r) * intensity) as u8;
                    let g = (f32::from(g) * intensity) as u8;
                    let b = (f32::from(b) * intensity) as u8;

                    image.set(x as usize, y as usize, ppm::RGB::new(r, g, b));
                }
            }
        }
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {

    let mut z_buffer: [f32; WIDTH * HEIGHT] = [std::f32::MIN; WIDTH * HEIGHT];
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_mesh("african_head.obj", "african_head_diffuse.tga", "african_head_nm.png", &mut image, &mut z_buffer)?;

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
    file.write_all(String::from(&image).as_bytes())?;

    println!("Done!");
    Ok(())
}
