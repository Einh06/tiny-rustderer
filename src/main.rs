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

trait Shader {
    fn vertex(&mut self, face_index: usize) -> (Vec4f, Vec4f, Vec4f);
    fn fragment(&self, bar: Vec3f) -> (u8, u8, u8);
}

struct PhongShader<'a> {
    // Input to graphic pipeline
    light_dir: Vec3f,
    texture_map: &'a image::Image<u8>,
    spec_map: &'a image::Image<u8>,
    tangent_map: &'a image::Image<u8>,
    mesh: &'a obj::Mesh,

    trans_matrix: Mat44,


    // Output from Vertex for frag
    normals: [Vec3f; 3],
    tangents: [Vec3f; 3],
    uvs: [Vec2f; 3],
}

impl<'a> PhongShader<'a> {
    fn new(light_dir: Vec3f, trans_matrix: Mat44, mesh: &'a obj::Mesh, texture_map: &'a image::Image<u8>, spec_map: &'a image::Image<u8>, tangent_map: &'a image::Image<u8>) -> PhongShader<'a> {
        PhongShader { 
            light_dir,
            trans_matrix,
            mesh, 
            texture_map, 
            spec_map,   
            tangent_map,

            uvs: [Vec2f::new(0.0, 0.0); 3], 
            normals: [Vec3f::new(0.0, 0.0, 0.0); 3],
            tangents: [Vec3f::new(0.0, 0.0, 0.0); 3],

        }
    }
}

impl<'a> Shader for PhongShader<'a> {

    fn vertex(&mut self, face_index: usize) -> (Vec4f, Vec4f, Vec4f) {
        let face = &self.mesh.faces[face_index];
        let (v1, t1, n1) = face[0];
        let (v2, t2, n2) = face[1];
        let (v3, t3, n3) = face[2];

        self.normals    = [self.mesh.normals[n1], self.mesh.normals[n2], self.mesh.normals[n3]];
        self.uvs        = [self.mesh.texcoord[t1], self.mesh.texcoord[t2], self.mesh.texcoord[t3]];
        self.tangents   = [self.mesh.tangents[v1], self.mesh.tangents[v2], self.mesh.tangents[v3]];

        let v1_transformed = self.trans_matrix * Vec4f::from_vec3f(self.mesh.vertices[v1], 1.0);
        let v2_transformed = self.trans_matrix * Vec4f::from_vec3f(self.mesh.vertices[v2], 1.0);
        let v3_transformed = self.trans_matrix * Vec4f::from_vec3f(self.mesh.vertices[v3], 1.0);

        (
            v1_transformed,
            v2_transformed,
            v3_transformed,
        )
    }

    fn fragment(&self, bar: Vec3f) -> (u8, u8, u8) {

        let uv = self.uvs[0] * bar.x + self.uvs[1] * bar.y + self.uvs[2] * bar.z;

        let bn = (self.normals[0] * bar.x + self.normals[1] * bar.y + self.normals[2] * bar.z).normalized();
        let tangent = (self.tangents[0] * bar.x + self.tangents[1] * bar.y + self.tangents[2] * bar.z).normalized();
        let bitangent = bn.cross(tangent).normalized();
            
        let tbn = Mat33::from_col_vec(tangent, bitangent, bn);
        let tbn_inv = tbn.transposed();

        let (nx, ny, nz) = texture(self.tangent_map, uv);
        let get_normal_value = |c| {f32::from(c) / 255.0 * 2.0 - 1.0};

        let bn = Vec3f::new(get_normal_value(nx), get_normal_value(ny), get_normal_value(nz));
        let light_dir_tangentspace = tbn_inv * self.light_dir;

        let diffuse = bn.dot(light_dir_tangentspace).max(0.0);

        let reflected_dir = (bn * (bn.dot(light_dir_tangentspace) * 2.0) - light_dir_tangentspace).normalized();
        let (spec, _, _) = texture(self.spec_map, uv);
        let spec = reflected_dir.z.max(0.0).powf(f32::from(spec));

        let (r, g, b) = texture(self.texture_map, uv);
        let (r, g, b) = (f32::from(r), f32::from(g), f32::from(b));

        let light_calc = |c: f32| -> u8 {
            (c * (diffuse + 0.6 * spec)) as u8
        };

        (light_calc(r), light_calc(g), light_calc(b))
    }
}


fn render_mesh_shader(mesh: &obj::Mesh, shader: &mut Shader, z_buffer: &mut Vec<f32>, image: &mut ppm::Image) {

    for index in 0..mesh.faces.len() {
        let (v1, v2, v3) = shader.vertex(index);
        let (v1_hom, v2_hom, v3_hom) = (v1.homogenize(), v2.homogenize(), v3.homogenize());

        let xmin = v1_hom.x.min(v2_hom.x.min(v3_hom.x)).max(0.0) as usize;
        let ymin = v1_hom.y.min(v2_hom.y.min(v3_hom.y)).max(0.0) as usize;
        let xmax = v1_hom.x.max(v2_hom.x.max(v3_hom.x)).min(FWIDTH - 1.0) as usize;
        let ymax = v1_hom.y.max(v2_hom.y.max(v3_hom.y)).min(FHEIGHT - 1.0) as usize;

        for y in  ymin..=ymax {
            for x in xmin..=xmax {
                let p = Vec2f::new(x as f32, y as f32);
                let bar = barycenter(v1_hom.xy(), v2_hom.xy(), v3_hom.xy(), p);

                if bar.x < 0.0 || bar.y < 0.0 || bar.z < 0.0 { continue; }

                let pos = v1 * bar.x + v2 * bar.y + v3 * bar.z;
                let fragment_depth = pos.z / pos.w;
                let zb = &mut z_buffer[(y * WIDTH) + x];

                if *zb > fragment_depth { continue; }
                *zb = fragment_depth;

                let (r, g, b) = shader.fragment(bar);
                image.set(x, y, ppm::RGB::new(r, g, b));
            }
        }
    }
}

fn render_scene(
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

    let texture_map = load_image_with_name(texture_name);
    let _normal_map = load_image_with_name(normal_map_name);
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
    let light_dir_worldspace = Vec3f::new(1.0, 1.0, 1.0).normalized();

    let camera_from_world = Mat44::lookat(eye, center, up);
    let view_from_camera = Mat44::projection(-1.0 / (eye - center).length());
    let screen_from_view = Mat44::viewport( FWIDTH / 8.0, FHEIGHT / 8.0, (FWIDTH * 3.0) / 4.0, (FHEIGHT * 3.0) / 4.0, MAX_DEPTH,);

    let screen_from_world = screen_from_view * view_from_camera * camera_from_world;

    let mut phong_shader = PhongShader::new(light_dir_worldspace, screen_from_world, &mesh, &texture_map, &spec_map, &tangent_map);

    render_mesh_shader(&mesh, &mut phong_shader, &mut z_buffer, image);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut image = ppm::Image::new(WIDTH, HEIGHT);

    render_scene(
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
