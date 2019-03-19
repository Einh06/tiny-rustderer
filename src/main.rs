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

fn barycenter(t: &[Vec3f; 3], p: Vec3f) -> Vec3f {
    let v1 = Vec3f { 
        x: (t[2].x - t[0].x),
        y: (t[1].x - t[0].x),
        z: (t[0].x - p.x),
    };

    let v2 = Vec3f {
        x: (t[2].y - t[0].y),
        y: (t[1].y - t[0].y),
        z: (t[0].y - p.y),
    };

    let u = v1.cross(v2);
    if u.z.abs() < 1_f32 { Vec3f::new(-1_f32, 1_f32, 1_f32) }
    else { Vec3f::new(1_f32 - (u.x+u.y) / u.z, u.y / u.z, u.x / u.z) }
}

fn triangle(t: &[Vec3f; 3], uv: &[Vec3f; 3],  texture: &image::Image<u8>, intensity: f32, image: &mut ppm::Image,  z_buffer: &mut [f32]) {
    
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

    println!("triangle: {:?}", t);
    for y in iymin..=iymax {
        for x in ixmin..=ixmax {
            let xu = x as usize;
            let yu = y as usize;

            let mut p = Vec3f::new(x as f32, y as f32, 0.0);

            let bar = barycenter(t, p);
            println!("  point {:?}, barycenter {:?}, ", p, bar);
            if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 { continue }

            p.z = t[0].z * bar.x + t[1].z * bar.y + t[2].z * bar.z;
            let mut zb = &mut z_buffer[(yu*WIDTH)+xu];

            if *zb < p.z { 
                *zb = p.z;

                let vtx = bar.x * uv[0].x + bar.y * uv[1].x + bar.z * uv[2].x;
                let vty = bar.x * uv[0].y + bar.y * uv[1].y + bar.z * uv[2].y;

                let u = (vtx * FWIDTH) as usize;
                let v = (image.height - 1) - (vty * FHEIGHT) as usize; //flipped vertically

                let r = texture.data[((v * image.width + u) * 3) + 0]; // RGB
                let g = texture.data[((v * image.width + u) * 3) + 1]; // RGB
                let b = texture.data[((v * image.width + u) * 3) + 2]; // RGB

                let r = (r as f32 * intensity) as u8;
                let g = (g as f32 * intensity) as u8;
                let b = (b as f32 * intensity) as u8;

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

    let projection = Mat44::projection(Vec3f::new(0.0, 0.0, 3.0));
    let viewport = Mat44::viewport(FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, 255.0);

    let pre_mult_mat = viewport * projection;

    for chunk in mesh.faces.chunks(3) {
        let (i1, t1, _) = chunk[0];
        let (i2, t2, _) = chunk[1];
        let (i3, t3, _) = chunk[2];

        let v1 = Vec3f::from(mesh.vertices[i1 as usize]);
        let v2 = Vec3f::from(mesh.vertices[i2 as usize]);
        let v3 = Vec3f::from(mesh.vertices[i3 as usize]);

        let uv1 = Vec3f::from(mesh.texcoord[t1 as usize]);
        let uv2 = Vec3f::from(mesh.texcoord[t2 as usize]);
        let uv3 = Vec3f::from(mesh.texcoord[t3 as usize]);

        let sv1 = Vec3f::from(pre_mult_mat * Vec4f::from(v1));
        let sv2 = Vec3f::from(pre_mult_mat * Vec4f::from(v2));
        let sv3 = Vec3f::from(pre_mult_mat * Vec4f::from(v3));

        let n = (v3 - v1).cross(v2 - v1).normalized();
        let dot = n.dot(Vec3f::new(0_f32, 0_f32, -1_f32));

        if dot > 0_f32 {
            let t: [Vec3f; 3] = [sv1, sv2, sv3];
            let uv: [Vec3f; 3] = [uv1, uv2, uv3];
            triangle(&t, &uv, &texture, dot, image, z_buffer);
        }
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {

    let mut z_buffer: [f32; WIDTH * HEIGHT] = [std::f32::MIN; WIDTH * HEIGHT];
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    {
        render_mesh("african_head.obj", "african_head_diffuse.tga", &mut image, &mut z_buffer)?;
    }
   
    /*
    {
        let t0 = [Vec2i::new(10, 70), Vec2i::new(50, 160), Vec2i::new(70, 80)];
        let t1 = [Vec2i::new(180, 50), Vec2i::new(150, 1), Vec2i::new(70, 180)];
        let t2 = [Vec2i::new(180, 150), Vec2i::new(120, 160), Vec2i::new(130, 180)];


        triangle(&t0, &mut image, ppm::RGB::red());
        triangle(&t1, &mut image, ppm::RGB::white());
        triangle(&t2, &mut image, ppm::RGB::green());
    }
    */

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
