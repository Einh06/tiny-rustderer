extern crate stb_image;
mod math;
mod obj;
mod ppm;

use math::{Mat33, Mat44, Vec2f, Vec3f, Vec4f};
use stb_image::image;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};

use image::LoadResult::{Error, ImageF32, ImageU8};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const FWIDTH: f32 = WIDTH as f32;
const FHEIGHT: f32 = HEIGHT as f32;

const MAX_DEPTH: f32 = 255.0;

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
    if u.z.abs() < 1_f32 {
        Vec3f::new(-1_f32, 1_f32, 1_f32)
    } else {
        Vec3f::new(1_f32 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

fn data_from_uv(image: &image::Image<u8>, uv: Vec2f) -> (u8, u8, u8) {
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

fn render_mesh(
    filename: &str,
    texture_name: &str,
    normal_map_name: &str,
    tangent_map_name: &str,
    specular_map_name: &str,
    image: &mut ppm::Image,
    z_buffer: &mut [f32],
) -> std::io::Result<()> {

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

    let texture = load_image_with_name(texture_name);
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

    let screen_from_view = Mat44::viewport( FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, MAX_DEPTH, );
    let light_dir_worldspace = Vec3f::new(1.0, 1.0, 1.0).normalized();

    let lightspace_from_worldspace = Mat44::lookat(light_dir_worldspace, center, up);
    let lightview_from_lightspace = Mat44::projection(0.0);
    let lightscreen_from_worldspace = screen_from_view * lightview_from_lightspace * lightspace_from_worldspace;

    let mut depth_image = ppm::Image::new(image.width, image.height);
    {
        //Shadow Map pass
        for face in &mesh.faces {
            let (i1, _, _) = face[0];
            let (i2, _, _) = face[1];
            let (i3, _, _) = face[2];

            let (v1_worldspace, v2_worldspace, v3_worldspace) =
                (mesh.vertices[i1], mesh.vertices[i2], mesh.vertices[i3]);

            let v1_lightscreen = (lightscreen_from_worldspace * Vec4f::from(v1_worldspace)).homogenize();
            let v2_lightscreen = (lightscreen_from_worldspace * Vec4f::from(v2_worldspace)).homogenize();
            let v3_lightscreen = (lightscreen_from_worldspace * Vec4f::from(v3_worldspace)).homogenize();

            let xmin = v1_lightscreen.x.min(v2_lightscreen.x.min(v3_lightscreen.x)).max(0.0) as usize;
            let ymin = v1_lightscreen.y.min(v2_lightscreen.y.min(v3_lightscreen.y)).max(0.0) as usize;
            let xmax = v1_lightscreen.x.max(v2_lightscreen.x.max(v3_lightscreen.x)).min(FWIDTH - 1.0) as usize;
            let ymax = v1_lightscreen.y.max(v2_lightscreen.y.max(v3_lightscreen.y)).min(FHEIGHT - 1.0) as usize;

            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let mut p = Vec3f::new(x as f32, y as f32, 0.0);
                    let bar = barycenter(v1_lightscreen, v2_lightscreen, v3_lightscreen, p);
                    if bar.x < 0.0 || bar.y < 0.0 || bar.z < 0.0 {
                        continue;
                    }

                    p.z = bar.x * v1_lightscreen.z + bar.y * v2_lightscreen.z + bar.z * v3_lightscreen.z;
                    let mut zb = &mut z_buffer[(y * WIDTH) + x];
                    if *zb < p.z {
                        *zb = p.z;

                        println!("z: {}", p.z);

                        // need to map 
                        let color = ppm::RGB::grey(p.z);
                        depth_image.set(x, y, color);
                    }
                }
            }
        }
    }

    let mut depth_buffer_path = resource_dir.clone();
    depth_buffer_path.push("depth.ppm");

    let mut depth_file = File::create(depth_buffer_path.as_path())?;

    println!("Writing to depth buffer");
    depth_file.write_all(String::from(&depth_image).as_bytes())?;


    let mut shadow_map = vec![0_f32; depth_image.width * depth_image.height];
    shadow_map.copy_from_slice(z_buffer);

    for v in &mut z_buffer.into_iter() { *v = std::f32::MIN; }

    {
        // Scene render pass
        let camera_from_world = Mat44::lookat(eye, center, up);
        let view_from_camera = Mat44::projection(-1.0 / (eye - center).length());
        let screen_from_world = screen_from_view * view_from_camera * camera_from_world;
        let screen_from_world_it = screen_from_world.inverse().transposed();

        let normalizing_matrix = Mat44::new(0.5, 0.0, 0.0, 0.5,
                                            0.0, 0.5, 0.0, 0.5,
                                            0.0, 0.0, 0.5, 0.5,
                                            0.0, 0.0, 0.0, 1.0,);

        // Model passs
        for face in mesh.faces {
            let (i1, t1, n1) = face[0];
            let (i2, t2, n2) = face[1];
            let (i3, t3, n3) = face[2];

            let (v1_worldspace, v2_worldspace, v3_worldspace) =
                (mesh.vertices[i1], mesh.vertices[i2], mesh.vertices[i3]);

            let v1_screenspace = (screen_from_world * Vec4f::from(v1_worldspace)).homogenize();
            let v2_screenspace = (screen_from_world * Vec4f::from(v2_worldspace)).homogenize();
            let v3_screenspace = (screen_from_world * Vec4f::from(v3_worldspace)).homogenize();

            let (uv1, uv2, uv3) = (mesh.texcoord[t1], mesh.texcoord[t2], mesh.texcoord[t3]);
            let (n1_worldspace, n2_worldspace, n3_worldspace) =
                (mesh.normals[n1], mesh.normals[n2], mesh.normals[n3]);

            // bounding box for triangle
            let ixmin = v1_screenspace.x.min(v2_screenspace.x.min(v3_screenspace.x)).max(0.0) as usize;
            let iymin = v1_screenspace.y.min(v2_screenspace.y.min(v3_screenspace.y)).max(0.0) as usize;
            let ixmax = v1_screenspace.x.max(v2_screenspace.x.max(v3_screenspace.x)).min(FWIDTH - 1.0) as usize;
            let iymax = v1_screenspace.y.max(v2_screenspace.y.max(v3_screenspace.y)).min(FHEIGHT - 1.0) as usize;

            for y in iymin..=iymax {
                for x in ixmin..=ixmax {
                    let mut p = Vec3f::new(x as f32, y as f32, 0.0);

                    let bar = barycenter(v1_screenspace, v2_screenspace, v3_screenspace, p);
                    if bar.x < 0_f32 || bar.y < 0_f32 || bar.z < 0_f32 {
                        continue;
                    }

                    p.z = v1_screenspace.z * bar.x
                        + v2_screenspace.z * bar.y
                        + v3_screenspace.z * bar.z;
                    let mut zb = &mut z_buffer[(y * WIDTH) + x];

                    if *zb < p.z {
                        *zb = p.z;

                        let v1_worldspace = (screen_from_world_it * Vec4f::from(v1_screenspace)).homogenize();
                        let v2_worldspace = (screen_from_world_it * Vec4f::from(v2_screenspace)).homogenize();
                        let v3_worldspace = (screen_from_world_it * Vec4f::from(v3_screenspace)).homogenize();

                        let pos_worldspace = v1_worldspace * bar.x + v2_worldspace * bar.y + v3_worldspace * bar.z;
                        let pos_lightspace = lightspace_from_worldspace * Vec4f::from(pos_worldspace);
                        let pos_lightscreen = (normalizing_matrix * pos_lightspace).homogenize();
                        let depth_x = (pos_lightscreen.x * (depth_image.width as f32)) as usize;
                        let depth_y = (pos_lightscreen.y * (depth_image.height as f32)) as usize;
                        let depth_z = shadow_map[(depth_y * depth_image.width) + depth_x];

                        if pos_lightscreen.z < depth_z {
                            image.set(x, y, ppm::RGB::red());
                            continue;
                        }

                        // If behind something in shadow map, donc calculate light

                        let u = bar.x * uv1.x + bar.y * uv2.x + bar.z * uv3.x;
                        let v = bar.x * uv1.y + bar.y * uv2.y + bar.z * uv3.y;
                        let uv = Vec2f::new(u, v);

                        let bn = Vec3f::new(
                            bar.x * n1_worldspace.x + bar.y * n2_worldspace.x + bar.z * n3_worldspace.x,
                            bar.x * n1_worldspace.y + bar.y * n2_worldspace.y + bar.z * n3_worldspace.y,
                            bar.x * n1_worldspace.z + bar.y * n2_worldspace.z + bar.z * n3_worldspace.z,
                        ).normalized();

                        let (diffuse, spec) = if true {
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

                            let (nx, ny, nz) = data_from_uv(&tangent_map, uv);

                            let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
                            let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
                            let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

                            let l_tangentspace = (tbn_inv * light_dir_worldspace).normalized();
                            let n_tangentspace = Vec3f::new(nx, ny, nz).normalized();
                            let diffuse = n_tangentspace.dot(l_tangentspace).max(0.0);

                            let reflected_dir = (n_tangentspace
                                * (n_tangentspace.dot(l_tangentspace) * 2.0)
                                - l_tangentspace)
                                .normalized();

                            let (spec, _, _) = data_from_uv(&spec_map, uv);
                            let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                            (diffuse, spec)
                        } else if true {
                            let (nx, ny, nz) = data_from_uv(&normal_map, uv);
                            let nx = (f32::from(nx) / 255.0) * 2.0 - 1.0;
                            let ny = (f32::from(ny) / 255.0) * 2.0 - 1.0;
                            let nz = (f32::from(nz) / 255.0) * 2.0 - 1.0;

                            let bn = Vec3f::new(nx, ny, nz).normalized();
                            let diffuse = bn.dot(light_dir_worldspace).max(0.0);

                            let reflected_dir = (bn * (bn.dot(light_dir_worldspace) * 2.0) - light_dir_worldspace).normalized();

                            let (spec, _, _) = data_from_uv(&spec_map, uv);
                            let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

                            (diffuse, spec)
                        } else {
                            gouraud_shading(bn, light_dir_worldspace, uv, &spec_map)
                        };

                        let ambient_color = 5_u8;

                        let (r, g, b) = data_from_uv(&texture, uv);
                        let (r, g, b) = (
                            (f32::from(r) * (diffuse + 0.6 * spec)) as u8,
                            (f32::from(g) * (diffuse + 0.6 * spec)) as u8,
                            (f32::from(b) * (diffuse + 0.6 * spec)) as u8
                        );

                        let r = ambient_color.saturating_add(r);
                        let g = ambient_color.saturating_add(g);
                        let b = ambient_color.saturating_add(b);

                        image.set(x, y, ppm::RGB::new(r, g, b));
                    }
                }
            }
        }
    }

    Ok(())
}

fn gouraud_shading(bn: Vec3f, light_dir: Vec3f, uv: Vec2f, spec_map: &image::Image<u8>) -> (f32, f32) {
    let diffuse = bn.dot(light_dir).max(0.0);

    let reflected_dir = (bn * (bn.dot(light_dir) * 2.0) - light_dir).normalized();
    let (spec, _, _) = data_from_uv(&spec_map, uv);
    let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

    (diffuse, spec)
}

fn main() -> std::io::Result<()> {
    let mut z_buffer: [f32; WIDTH * HEIGHT] = [std::f32::MIN; WIDTH * HEIGHT];
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_mesh(
        "african_head.obj",
        "african_head_diffuse.tga",
        "african_head_nm.tga",
        "african_head_nm_tangent.tga",
        "african_head_spec.tga",
        &mut image,
        &mut z_buffer,
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
