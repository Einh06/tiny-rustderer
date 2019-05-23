extern crate stb_image;
mod math;
mod ppm;
mod obj;

use std::fs::{DirBuilder, File};
use std::io::{Write, Read};
use stb_image::image;
use math::{Vec2f, Vec3f, Vec4f, Mat33, Mat44};

use image::LoadResult::{Error, ImageU8, ImageF32};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;

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

fn image_get_from_uv(image: &image::Image<u8>, uv: Vec2f) -> (u8, u8, u8) {
    let fnwidth  = image.width  as f32;
    let fnheight = image.height as f32;
    let nu = (uv.x * fnwidth) as usize;
    let nv = ((1.0 - uv.y) * fnheight) as usize; //flipped vertically

    let pixel_index = (nv * image.width + nu) * image.depth;

    (image.data[pixel_index + 0], image.data[pixel_index + 1], image.data[pixel_index + 2])
}

fn render_mesh(filename: &str, texture_name: &str, normal_map_name: &str, tangent_map_name: &str, specular_map_name: &str, image: &mut ppm::Image, z_buffer: &mut [f32]) -> std::io::Result<()> {
    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");
    
    let texture = {
        let mut texture_path = resource_dir.clone();
        texture_path.push(texture_name);
        let image = match image::load(texture_path.as_path()) {
            Error(str) => panic!(str),
            ImageU8(image) => image,
            ImageF32(_image) => panic!("Wrong image format"),
        };
        image
    };

    let normal_map = {
        let mut normal_map_path = resource_dir.clone();
        normal_map_path.push(normal_map_name);
        let image = match image::load(normal_map_path.as_path()) {
            Error(str) => panic!(str),
            ImageU8(image) => image,
            ImageF32(_image) => panic!("Wrong image format"),
        };
        image
    };

    let tangent_map = {
        let mut tangent_map_path = resource_dir.clone();
        tangent_map_path.push(tangent_map_name);
        let image = match image::load(tangent_map_path.as_path()) {
            Error(str) => panic!(str),
            ImageU8(image) => image,
            ImageF32(_image) => panic!("Wrong image format"),
        };
        image
    };

    let spec_map = {
        let mut specular_map_path = resource_dir.clone();
        specular_map_path.push(specular_map_name);
        let image = match image::load(specular_map_path.as_path()) {
            Error(str) => panic!(str),
            ImageU8(image) => image,
            ImageF32(_image) => panic!("Wrong image format"),
        };
        image

    };

    let mesh = { 
        resource_dir.push(filename);

        println!("Opening model file");
        let mut file = File::open(resource_dir.as_path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        println!("loading mesh content");

        obj::Mesh::load(&content[..])
    };

    let eye     = Vec3f::new(1.0, 1.0, 3.0);
    let center  = Vec3f::new(0.0, 0.0, 0.0);
    let up      = Vec3f::new(0.0, 1.0, 0.0);

    let camera_from_world   = Mat44::lookat(eye, center, up);
    let view_from_camera    = Mat44::projection(-1.0/(eye - center).length());
    let screen_from_view    = Mat44::viewport(FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, 255.0);

    let screen_from_world = screen_from_view * view_from_camera * camera_from_world;

    let light_dir_worldspace = Vec3f::new(1.0, 1.0, 1.0).normalized();

    for face in mesh.faces {
        let (i1, t1, n1) = face[0];
        let (i2, t2, n2) = face[1];
        let (i3, t3, n3) = face[2];

        let (v1_worldspace, v2_worldspace, v3_worldspace) = (mesh.vertices[i1], mesh.vertices[i2], mesh.vertices[i3]);

        let v1_screenspace = (screen_from_world * Vec4f::from(v1_worldspace)).homogenize();
        let v2_screenspace = (screen_from_world * Vec4f::from(v2_worldspace)).homogenize();
        let v3_screenspace = (screen_from_world * Vec4f::from(v3_worldspace)).homogenize();

        let (uv1, uv2, uv3) = (mesh.texcoord[t1], mesh.texcoord[t2], mesh.texcoord[t3]);

        let (n1_worldspace, n2_worldspace, n3_worldspace) = (mesh.normals[n1], mesh.normals[n2], mesh.normals[n3]);

        // bounding box for triangle
        let ixmin = v1_screenspace.x.min(v2_screenspace.x.min(v3_screenspace.x)).max(0.0) as usize;
        let iymin = v1_screenspace.y.min(v2_screenspace.y.min(v3_screenspace.y)).max(0.0) as usize;
        let ixmax = v1_screenspace.x.max(v2_screenspace.x.max(v3_screenspace.x)).min(FWIDTH - 1.0) as usize;
        let iymax = v1_screenspace.y.max(v2_screenspace.y.max(v3_screenspace.y)).min(FHEIGHT - 1.0) as usize;

        for y in iymin..=iymax {
            for x in ixmin..=ixmax {
                let mut p = Vec3f::new(x as f32, y as f32, 0.0);

                let bar = barycenter(v1_screenspace, v2_screenspace, v3_screenspace, p);
                if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 { continue }

                p.z = v1_screenspace.z * bar.x + v2_screenspace.z * bar.y + v3_screenspace.z * bar.z;
                let mut zb = &mut z_buffer[(y*WIDTH)+x];

                if *zb < p.z { 
                    *zb = p.z;

                    let u = bar.x * uv1.x + bar.y * uv2.x + bar.z * uv3.x;
                    let v = bar.x * uv1.y + bar.y * uv2.y + bar.z * uv3.y;
                    let uv = Vec2f::new(u, v);

                    let bn = Vec3f::new(bar.x * n1_worldspace.x + bar.y * n2_worldspace.x + bar.z * n3_worldspace.x,
                                        bar.x * n1_worldspace.y + bar.y * n2_worldspace.y + bar.z * n3_worldspace.y,
                                        bar.x * n1_worldspace.z + bar.y * n2_worldspace.z + bar.z * n3_worldspace.z,).normalized();

                    let (diffuse, spec) = if true {

                        let tan1 = mesh.tangents[i1];
                        let tan2 = mesh.tangents[i2];
                        let tan3 = mesh.tangents[i3];

                        let tangent = Vec3f::new( bar.x * tan1.x + bar.y * tan2.x + bar.z * tan3.x,
                                                  bar.x * tan1.y + bar.y * tan2.y + bar.z * tan3.y,
                                                  bar.x * tan1.z + bar.y * tan2.z + bar.z * tan3.z,).normalized();
                        let bitangent = bn.cross(tangent);

                        let tbn = Mat33::from_col_vec(tangent, bitangent, bn);
                        let tbn_inv = tbn.transposed();

                        let (nx, ny, nz) = image_get_from_uv(&tangent_map, uv);

                        let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
                        let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
                        let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

                        let l_tangentspace = (tbn_inv * light_dir_worldspace).normalized();
                        let n_tangentspace = Vec3f::new(nx, ny, nz).normalized();
                        let diffuse = n_tangentspace.dot(l_tangentspace).max(0.0);

                        let reflected_dir = (n_tangentspace * (n_tangentspace.dot(l_tangentspace) * 2.0) - l_tangentspace).normalized();

                        let (spec, _, _) = image_get_from_uv(&spec_map, uv);
                        let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                        (diffuse, spec)
                    } else if true {

                        let (nx, ny, nz) = image_get_from_uv(&normal_map, uv);
                        let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
                        let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
                        let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

                        let bn = Vec3f::new(nx, ny, nz).normalized();
                        let diffuse = bn.dot(light_dir_worldspace).max(0.0);

                        let reflected_dir = (bn * (bn.dot(light_dir_worldspace) * 2.0) - light_dir_worldspace).normalized();

                        let (spec, _, _) = image_get_from_uv(&spec_map, uv);
                        let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                        (diffuse, spec)

                    } else {
                        let diffuse = bn.dot(light_dir_worldspace).max(0.0);

                        let reflected_dir = (bn * (bn.dot(light_dir_worldspace) * 2.0) - light_dir_worldspace).normalized();
                        let (spec, _, _) = image_get_from_uv(&spec_map, uv);
                        let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                        (diffuse, spec)
                    };

                    let ambient_color = 5_u8;

                    let (r, g, b) = image_get_from_uv(&texture, uv);

                    let r = ambient_color.saturating_add( (f32::from(r) * (diffuse + 0.6 * spec)) as u8);
                    let g = ambient_color.saturating_add( (f32::from(g) * (diffuse + 0.6 * spec)) as u8);
                    let b = ambient_color.saturating_add( (f32::from(b) * (diffuse + 0.6 * spec)) as u8);

                    image.set(x, y, ppm::RGB::new(r, g, b));
                }
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {

    let mut z_buffer: [f32; WIDTH * HEIGHT] = [std::f32::MIN; WIDTH * HEIGHT];
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_mesh("african_head.obj", "african_head_diffuse.tga", "african_head_nm.tga", "african_head_nm_tangent.tga", "african_head_spec.tga", &mut image, &mut z_buffer)?;
    // render_mesh("diablo3_pose.obj", "diablo3_pose_diffuse.tga", "diablo3_pose_nm.tga", "diablo3_pose_nm_tangent.tga", &mut image, &mut z_buffer)?;

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
