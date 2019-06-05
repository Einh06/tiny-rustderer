extern crate stb_image;
mod math;
mod obj;
mod ppm;

use math::{Mat33, Mat44, Vec2f, Vec3f, Vec4f};
use stb_image::image;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};

use image::LoadResult::{Error, ImageF32, ImageU8};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;

const MAX_DEPTH: f32 = 2000.0;

#[allow(dead_code)]
fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut ppm::Image, color: ppm::RGB) {
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    let (x0, x1, y0, y1) = if steep {
        (y0, y1, x0, x1)
    } else {
        (x0, x1, y0, y1)
    }; // SWAP
    let (x0, x1, y0, y1) = if x0 > x1 {
        (x1, x0, y1, y0)
    } else {
        (x0, x1, y0, y1)
    }; // SWAP

    let (dx, dy) = (x1 - x0, y1 - y0);
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

fn barycenter(a: Vec2f, b: Vec2f, c: Vec2f, p: Vec2f) -> Vec3f {
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
    if u.z.abs() < 1_f32 {
        Vec3f::new(-1_f32, 1_f32, 1_f32)
    } else {
        Vec3f::new(1_f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

fn texture(image: &image::Image<u8>, uv: Vec2f) -> (u8, u8, u8) {
    let fnwidth = image.width as f32;
    let fnheight = image.height as f32;
    let nu = (uv.x * fnwidth) as usize;
    let nv = ((1.0 - uv.y) * fnheight) as usize; //flipped vertically

    let pixel_index = (nv * image.width + nu) * image.depth;

    (
        image.data[pixel_index + 0],
        image.data[pixel_index + 1],
        image.data[pixel_index + 2],
    )
}

fn gouraud_shading(bn: Vec3f, light_dir: Vec3f, uv: Vec2f, spec_map: &image::Image<u8>) -> (f32, f32) {
    let diffuse = bn.dot(light_dir).max(0.0);

    let reflected_dir = (bn * (bn.dot(light_dir) * 2.0) - light_dir).normalized();
    let (spec, _, _) = texture(&spec_map, uv);
    let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

    (diffuse, spec)
}

fn worldspace_normal_mapping(bn: Vec3f, light_dir: Vec3f, uv: Vec2f, normal_map: &image::Image<u8>, spec_map: &image::Image<u8>) -> (f32, f32) {
    let (nx, ny, nz) = texture(&normal_map, uv);
    let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
    let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
    let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

    let bn = Vec3f::new(nx, ny, nz).normalized();
    let diffuse = bn.dot(light_dir).max(0.0);

    let reflected_dir = (bn * (bn.dot(light_dir) * 2.0) - light_dir).normalized();

    let (spec, _, _) = texture(&spec_map, uv);
    let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

    (diffuse, spec)
}

fn render_mesh(
    filename: &str,
    texture_name: &str,
    normal_map_name: &str,
    tangent_map_name: &str,
    specular_map_name: &str,
    image: &mut ppm::Image,
) -> std::io::Result<()> {

    let mut z_buffer = vec![std::f32::MIN; image.width * image.height];

    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");

    let load_image_with_name = |n: &str| -> image::Image<u8> {
        println!("Loading {}", n);
        let mut path = resource_dir.clone();
        path.push(n);
        match image::load(path.as_path()) {
            Error(str) => panic!(str),
            ImageU8(image) => image,
            ImageF32(_image) => panic!("Wrong image format"),
        }
    };

    let diffuse_map = load_image_with_name(texture_name);
    let normal_map = load_image_with_name(normal_map_name);
    let tangent_map = load_image_with_name(tangent_map_name);
    let spec_map = load_image_with_name(specular_map_name);

    let mesh = {
        let mut mesh_path = resource_dir.clone();
        mesh_path.push(filename);

        println!("Opening model file");
        let mut file = File::open(mesh_path.as_path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        println!("loading mesh content");

        obj::Mesh::load(&content[..])
    };


    let eye = Vec3f::new(1.0, 1.0, 4.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);
    let up = Vec3f::new(0.0, 1.0, 0.0);

    let light_dir_worldspace = Vec3f::new(1.0, 1.0, 0.0).normalized();

    let lightspace_from_worldspace = Mat44::lookat(light_dir_worldspace, center, up);
    let lightview_from_lightspace = Mat44::projection(0.0);

    let screen_from_view = Mat44::viewport( FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, MAX_DEPTH,);

    let lightport_from_worldspace = screen_from_view * lightview_from_lightspace * lightspace_from_worldspace;

    let mut depth_image = ppm::Image::new(image.width, image.height);

    { //Shadow Map pass
        for face in &mesh.faces {
            let (i1, _, _) = face[0];
            let (i2, _, _) = face[1];
            let (i3, _, _) = face[2];

            let (v1_worldspace, v2_worldspace, v3_worldspace) =
                (mesh.vertices[i1], mesh.vertices[i2], mesh.vertices[i3]);

            let v1_lightport = lightport_from_worldspace * Vec4f::from_vec3f(v1_worldspace, 1.0);
            let v2_lightport = lightport_from_worldspace * Vec4f::from_vec3f(v2_worldspace, 1.0);
            let v3_lightport = lightport_from_worldspace * Vec4f::from_vec3f(v3_worldspace, 1.0);

            let v1_lightport_hom = v1_lightport.homogenize();
            let v2_lightport_hom = v2_lightport.homogenize();
            let v3_lightport_hom = v3_lightport.homogenize();

            let xmin = v1_lightport_hom.x.min(v2_lightport_hom.x.min(v3_lightport_hom.x)).max(0.0) as usize;
            let ymin = v1_lightport_hom.y.min(v2_lightport_hom.y.min(v3_lightport_hom.y)).max(0.0) as usize;
            let xmax = v1_lightport_hom.x.max(v2_lightport_hom.x.max(v3_lightport_hom.x)).min(FWIDTH - 1.0) as usize;
            let ymax = v1_lightport_hom.y.max(v2_lightport_hom.y.max(v3_lightport_hom.y)).min(FHEIGHT - 1.0) as usize;

            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let mut p = Vec2f::new(x as f32, y as f32);

                    let v1 = v1_lightport_hom.xy();
                    let v2 = v2_lightport_hom.xy();
                    let v3 = v3_lightport_hom.xy();

                    let bar = barycenter(v1, v2, v3, p);
                    if bar.x < 0.0 || bar.y < 0.0 || bar.z < 0.0 {
                        continue;
                    }

                    let pos_lightport = v1_lightport * bar.x + v2_lightport * bar.y + v3_lightport * bar.z;
                    let fragment_depth = (pos_lightport.z/pos_lightport.w).floor();
                    let mut zb = &mut z_buffer[(y * WIDTH) + x];
                    if *zb > fragment_depth { continue; }
                    *zb = fragment_depth;

                    pos_lightport.homogenize();
                    let color = ppm::RGB::grey(fragment_depth / MAX_DEPTH);
                    depth_image.set(x, y, color);
                }
            }
        }
    }

    let mut depth_buffer_path = resource_dir.clone();
    depth_buffer_path.push("depth.ppm");

    let mut depth_file = File::create(depth_buffer_path.as_path())?;

    println!("Writing to depth buffer");
    depth_file.write_all(String::from(&depth_image).as_bytes())?;

    let mut shadow_map = vec![0_f32; image.width * image.height];
    shadow_map.clone_from_slice(&z_buffer[..]);

    let mut z_buffer = vec![std::f32::MIN; image.width * image.height];

    { // Scene render pass
        let camera_from_world = Mat44::lookat(eye, center, up);
        let view_from_camera = Mat44::projection(-1.0 / (eye - center).length());
        let screen_from_world = screen_from_view * view_from_camera * camera_from_world;
        let lightport_from_viewport = lightport_from_worldspace * screen_from_world.inverse();

        // Model passs
        for face in mesh.faces {
            let (i1, t1, n1) = face[0];
            let (i2, t2, n2) = face[1];
            let (i3, t3, n3) = face[2];

            let (v1_worldspace, v2_worldspace, v3_worldspace) = 
                (Vec4f::from_vec3f(mesh.vertices[i1], 1.0), 
                 Vec4f::from_vec3f(mesh.vertices[i2], 1.0), 
                 Vec4f::from_vec3f(mesh.vertices[i3], 1.0));

            let v1_screenspace = screen_from_world * v1_worldspace;
            let v2_screenspace = screen_from_world * v2_worldspace;
            let v3_screenspace = screen_from_world * v3_worldspace;

            let v1_screenspace_hom = v1_screenspace.homogenize();
            let v2_screenspace_hom = v2_screenspace.homogenize();
            let v3_screenspace_hom = v3_screenspace.homogenize();

            let (uv1, uv2, uv3) = (mesh.texcoord[t1], mesh.texcoord[t2], mesh.texcoord[t3]);
            let (n1_worldspace, n2_worldspace, n3_worldspace) = (mesh.normals[n1], mesh.normals[n2], mesh.normals[n3]);

            // bounding box for triangle
            let ixmin = v1_screenspace_hom.x.min(v2_screenspace_hom.x.min(v3_screenspace_hom.x)).max(0.0) as usize;
            let iymin = v1_screenspace_hom.y.min(v2_screenspace_hom.y.min(v3_screenspace_hom.y)).max(0.0) as usize;
            let ixmax = v1_screenspace_hom.x.max(v2_screenspace_hom.x.max(v3_screenspace_hom.x)).min(FWIDTH - 1.0) as usize;
            let iymax = v1_screenspace_hom.y.max(v2_screenspace_hom.y.max(v3_screenspace_hom.y)).min(FHEIGHT - 1.0) as usize;

            for y in iymin..=iymax {
                for x in ixmin..=ixmax {
                    let mut p = Vec2f::new(x as f32, y as f32);

                    let bar = barycenter(v1_screenspace_hom.xy(), v2_screenspace_hom.xy(), v3_screenspace_hom.xy(), p);
                    if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 { continue; } 

                    let pos_screenspace = v1_screenspace * bar.x + v2_screenspace * bar.y + v3_screenspace * bar.z;

                    let fragment_depth = (pos_screenspace.z/pos_screenspace.w).floor();
                    let mut zb = &mut z_buffer[(y * WIDTH) + x];

                    if *zb > fragment_depth { continue; }
                    *zb = fragment_depth;

                    let pos_lightport = (lightport_from_viewport * pos_screenspace).homogenize();

                    let shadow_x = (pos_lightport.x) as usize;
                    let shadow_y = (pos_lightport.y * depth_image.width as f32) as usize;
                    let map_index = shadow_x + shadow_y;
                    let map_z = shadow_map[map_index];

                    // If behind something in shadow map, donc calculate light
                    let shadow = if map_z < (fragment_depth + 40.00) {
                        1.0
                    } else {
                        0.3
                    };

                    let shadow = 1.0;
                    
                    let u = bar.x * uv1.x + bar.y * uv2.x + bar.z * uv3.x;
                    let v = bar.x * uv1.y + bar.y * uv2.y + bar.z * uv3.y;
                    let uv = Vec2f::new(u, v);

                    let bn = Vec3f::new(
                        bar.x * n1_worldspace.x + bar.y * n2_worldspace.x + bar.z * n3_worldspace.x,
                        bar.x * n1_worldspace.y + bar.y * n2_worldspace.y + bar.z * n3_worldspace.y,
                        bar.x * n1_worldspace.z + bar.y * n2_worldspace.z + bar.z * n3_worldspace.z,
                    ).normalized();

                    let tan1 = mesh.tangents[i1];
                    let tan2 = mesh.tangents[i2];
                    let tan3 = mesh.tangents[i3];

                    let tangent = Vec3f::new(
                        bar.x * tan1.x + bar.y * tan2.x + bar.z * tan3.x,
                        bar.x * tan1.y + bar.y * tan2.y + bar.z * tan3.y,
                        bar.x * tan1.z + bar.y * tan2.z + bar.z * tan3.z,
                    )
                    .normalized();
                    let bitangent = bn.cross(tangent);

                    let tbn = Mat33::from_col_vec(tangent, bitangent, bn);
                    let tbn_inv = tbn.transposed();

                    let (nx, ny, nz) = texture(&tangent_map, uv);

                    let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
                    let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
                    let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

                    let l_tangentspace = (tbn_inv * light_dir_worldspace).normalized();
                    let n_tangentspace = Vec3f::new(nx, ny, nz).normalized();
                    let diffuse = n_tangentspace.dot(l_tangentspace).max(0.0);

                    let reflected_dir = (n_tangentspace * (n_tangentspace.dot(l_tangentspace) * 2.0) - l_tangentspace).normalized();

                    let (spec, _, _) = texture(&spec_map, uv);
                    let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                    let ambient_color = 20_u8;
                    let (r, g, b) = texture(&diffuse_map, uv);
                    let (r, g, b) = (
                        (f32::from(r) * shadow * (1.2 * diffuse + 0.6 * spec)) as u8,
                        (f32::from(g) * shadow * (1.2 * diffuse + 0.6 * spec)) as u8,
                        (f32::from(b) * shadow * (1.2 * diffuse + 0.6 * spec)) as u8
                    );

                    let r = ambient_color.saturating_add(r);
                    let g = ambient_color.saturating_add(g);
                    let b = ambient_color.saturating_add(b);

                    image.set(x, y, ppm::RGB::new(r, g, b));
                }
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_mesh(
        "african_head.obj",
        "african_head_diffuse.tga",
        "african_head_nm.tga",
        "african_head_nm_tangent.tga",
        "african_head_spec.tga",
        &mut image,
    )?;

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
