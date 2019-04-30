extern crate stb_image;
mod math;
mod ppm;
mod obj;

use std::fs::{DirBuilder, File};
use std::io::{Write, Read};
use stb_image::image;
use math::{Vec3f, Vec4f, Mat44};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
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

fn triangle(t: &[Vec3f; 3], n: &[Vec3f; 3], light_dir: Vec3f, uv: &[Vec3f; 3], texture: &image::Image<u8>, image: &mut ppm::Image,  z_buffer: &mut [f32]) {
    
    let xmin = t[0].x.min(t[1].x.min(t[2].x));
    let ymin = t[0].y.min(t[1].y.min(t[2].y));
    let xmax = t[0].x.max(t[1].x.max(t[2].x));
    let ymax = t[0].y.max(t[1].y.max(t[2].y));

    //Clamping
    let xmin = xmin.max(0_f32); 
    let xmax = xmax.min(FWIDTH - 1_f32);
    let ymin = ymin.max(0_f32);
    let ymax = ymax.min(FHEIGHT - 1_f32);

    let ixmin = xmin as i32;
    let ixmax = xmax as i32;
    let iymin = ymin as i32;
    let iymax = ymax as i32;

    for y in iymin..=iymax {
        for x in ixmin..=ixmax {
            let xu = x as usize;
            let yu = y as usize;

            let mut p = Vec3f::new(x as f32, y as f32, 0.0);

            let bar = barycenter(t[0], t[1], t[2], p);
            if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 { continue }

            p.z = t[0].z * bar.x + t[1].z * bar.y + t[2].z * bar.z;
            let mut zb = &mut z_buffer[(yu*WIDTH)+xu];

            if *zb < p.z { 
                *zb = p.z;

                let ftwidth = texture.width as f32;
                let ftheight = texture.height as f32;

                let nx = bar.x * n[0].x + bar.y * n[1].x + bar.z * n[2].x;
                let ny = bar.x * n[0].y + bar.y * n[1].y + bar.z * n[2].y;
                let nz = bar.x * n[0].z + bar.y * n[1].z + bar.z * n[2].z;
                let ndot = Vec3f::new(nx, ny, nz).normalized().dot(light_dir.normalized()).max(0.0);

                let vtx = bar.x * uv[0].x + bar.y * uv[1].x + bar.z * uv[2].x;
                let vty = bar.x * uv[0].y + bar.y * uv[1].y + bar.z * uv[2].y;

                let u = (vtx * (ftwidth - 1.0)) as usize;
                let v = (texture.height - 1) - ((vty * (ftheight - 1.0)) as usize); //flipped vertically

                let r = texture.data[((v * image.width + u) * 3) + 0]; // RGB
                let g = texture.data[((v * image.width + u) * 3) + 1]; // RGB
                let b = texture.data[((v * image.width + u) * 3) + 2]; // RGB

                let r = (r as f32 * ndot) as u8;
                let g = (g as f32 * ndot) as u8;
                let b = (b as f32 * ndot) as u8;

                image.set(x as usize, y as usize, ppm::RGB::new(r, g, b));
            }
        }
    }
}

fn render_mesh(filename: &str, texture_name: &str, image: &mut ppm::Image, z_buffer: &mut [f32]) -> std::io::Result<()> {
    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");
    let mut texture_path = resource_dir.clone();

    resource_dir.push(filename);
    texture_path.push(texture_name);

    let texture: image::Image<u8>;
    use image::LoadResult::{Error, ImageU8, ImageF32};
    match image::load(texture_path.as_path()) {
        Error(str) => panic!(str),
        ImageU8(image) => texture = image,
        ImageF32(_image) => panic!("Wrong image format"),
    };

    println!("Opening model file");
    let mut file = File::open(resource_dir.as_path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("loading mesh content");
    let mesh = obj::Mesh::load(&content[..]);

    let eye = Vec3f::new(-1.0, -1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);
    let up = Vec3f::new(0.0, 1.0, 0.0);

    let object_to_eye = Mat44::lookat(eye, center, up);
    let eye_to_view = Mat44::projection(-1.0/((eye - center).length()));
    let view_to_screen = Mat44::viewport(FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, 255.0);
    let object_to_view = eye_to_view * object_to_eye;
    let object_to_screen = view_to_screen * object_to_view;

    let inv_object_to_view = object_to_view.inverse().transposed();

    let light_dir = Vec3f::new(0.0, 0.0, 3.0).normalized();
    for chunk in mesh.faces.chunks(3) {
        let (i1, t1, n1) = chunk[0];
        let (i2, t2, n2) = chunk[1];
        let (i3, t3, n3) = chunk[2];

        let v1 = Vec3f::from(mesh.vertices[i1]);
        let v2 = Vec3f::from(mesh.vertices[i2]);
        let v3 = Vec3f::from(mesh.vertices[i3]);

        let sv1 = Vec3f::from(object_to_screen * Vec4f::from(v1));
        let sv2 = Vec3f::from(object_to_screen * Vec4f::from(v2));
        let sv3 = Vec3f::from(object_to_screen * Vec4f::from(v3));

        let uv1 = Vec3f::from(mesh.texcoord[t1]);
        let uv2 = Vec3f::from(mesh.texcoord[t2]);
        let uv3 = Vec3f::from(mesh.texcoord[t3]);

        let vn1 = Vec3f::from(inv_object_to_view * Vec4f::from(Vec3f::from(mesh.normals[n1])));
        let vn2 = Vec3f::from(inv_object_to_view * Vec4f::from(Vec3f::from(mesh.normals[n2])));
        let vn3 = Vec3f::from(inv_object_to_view * Vec4f::from(Vec3f::from(mesh.normals[n3])));


        let t: [Vec3f; 3] = [sv1, sv2, sv3];
        let uv: [Vec3f; 3] = [uv1, uv2, uv3];
        let vn: [Vec3f; 3] = [vn1, vn2, vn3];

        triangle(&t, &vn, light_dir, &uv, &texture, image, z_buffer);
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {

    let mut z_buffer: [f32; WIDTH * HEIGHT] = [std::f32::MIN; WIDTH * HEIGHT];
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_mesh("african_head.obj", "african_head_diffuse.tga", &mut image, &mut z_buffer)?;

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
