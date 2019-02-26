//@TODO(Florian): OBJ loader incomplete
use std::fs::{DirBuilder, File};
use std::io::{Write, Read};
use std::ops::{Add, Sub, Mul};
use std::cmp::{min, max};

mod ppm;
mod obj;

#[derive(Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2{ 
        Vec2 {x, y}
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, v: Vec2) -> Vec2 {
        Vec2::new( self.x + v.x, self.y + v.y )
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, v: Vec2) -> Vec2 {
        Vec2::new(self.x - v.x, self.y - v.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(((self.x as f32) * s) as i32, ((self.y as f32) * s) as i32)
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f {

    fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x,y,z}
    }

    fn cross(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
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

fn barycenter(t: &[Vec2; 3], p: Vec2) -> Vec3f {

    let v1 = Vec3f { 
        x: (t[2].x - t[0].x) as f32,
        y: (t[1].x - t[0].x) as f32,
        z: (t[0].x - p.x) as f32,
    };

    let v2 = Vec3f {
        x: (t[2].y - t[0].y) as f32,
        y: (t[1].y - t[0].y) as f32,
        z: (t[0].y - p.y) as f32,
    };

    let u = v1.cross(v2);
    if u.z.abs() < 1_f32 { return Vec3f::new(-1_f32, 1_f32, 1_f32); }
    Vec3f {x: 1_f32 - (u.x+u.y) / u.z, y: u.y / u.z, z: u.x / u.z }
}

fn triangle(t: &[Vec2; 3], image: &mut ppm::Image, color: ppm::RGB) {
    // Flat triangle, we don't care
    if t[0].y == t[1].y && t[0].y == t[2].y { return; }
    

    let xmin = min(t[0].x, min(t[1].x, t[2].x));
    let ymin = min(t[0].y, min(t[1].y, t[2].y));
    let xmax = max(t[0].x, max(t[1].x, t[2].x));
    let ymax = max(t[0].y, max(t[1].y, t[2].y));

    let xmin = max(0, xmin);
    let xmax = min(image.width as i32 - 1, xmax);
    let ymin = max(0, ymin);
    let ymax = min(image.height as i32 - 1, ymax);

    /*
    let (t0x, t0y) = (t0.x as f32, t0.y as f32);
    let (t1x, t1y) = (t1.x as f32, t1.y as f32);
    let (t2x, t2y) = (t2.x as f32, t2.y as f32);
    let inv_det = 1_f32 / ((t1y - t2y)*(t0x-t2x) + (t2x-t1x)*(t0y - t2y)).abs();

    for y in ymin..ymax {
        for x in xmin..xmax {

            let fx = x as f32;
            let fy = y as f32;

            let l1 = (((t1y - t2y)*(fx - t2x)) + ((t2x - t1x)*(fy - t2y))) * inv_det;
            let l2 = (((t2y - t0y)*(fx - t2x)) + ((t0x - t2x)*(fy - t2y))) * inv_det;
            let l3 = 1_f32 - l1 - l2;

            if (l1 + l2 + l3) - 1_f32 < EPS {
                image.set(x as usize, y as usize, color);
            }
        }
    }
    */
    for y in ymin..ymax {
        for x in xmin..xmax {
            let v = barycenter(t, Vec2 {x,y});
            if v.x < 0_f32 || v.y < 0_f32 || v.z < 0_f32 { continue }
            image.set(x as usize, y as usize, color);
        }
    }
    
    /*
    // order vertices by ascending y
    let (t0, t1) = if t0.y > t1.y {(t1, t0)} else {(t0, t1)};
    let (t0, t2) = if t0.y > t2.y {(t2, t0)} else {(t0, t2)};
    let (t1, t2) = if t1.y > t2.y {(t2, t1)} else {(t1, t2)};
    for y in t0.y..t1.y {
        let segment_height = t1.y-t0.y+1;
        let alpha = (y - t0.y) as f32 / total_height as f32;
        let beta = (y - t0.y) as f32 / segment_height as f32;

        let A = t0 + (t2 - t0) * alpha;
        let B = t0 + (t1 - t0) * beta;
        let (A, B) = if A.x > B.x {(B,A)} else {(A,B)};
        for x in A.x..B.x {
            image.set(x as usize, y as usize, color);
        }
    }
    for y in t1.y..t2.y {
        let segment_height = t2.y-t1.y+1;
        let alpha = (y - t0.y) as f32 / total_height as f32;
        let beta = (y - t1.y) as f32 / segment_height as f32;

        let A = t0 + (t2 - t0) * alpha;
        let B = t1 + (t2 - t1) * beta;
        let (A, B) = if A.x > B.x {(B,A)} else {(A,B)};
        for x in A.x..B.x {
            image.set(x as usize, y as usize, color);
        }
    }
    */
}


fn render_mesh(filename: &str, image: &mut ppm::Image) -> std::io::Result<()> {

    let fwidth = (image.width - 1) as f32;
    let fheight = (image.height - 1) as f32;

    let mut resource_dir = std::env::current_dir().unwrap();
    resource_dir.push("rsrc");
    resource_dir.push(filename);

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

        let t: [Vec2; 3] = [Vec2::new(x1,y1), Vec2::new(x2, y2), Vec2::new(x3, y3)];
        triangle(&t, image, ppm::RGB::white());
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {

    let width = 200;
    let height = 200;

    let mut image = ppm::Image::new(width, height);

    {
        render_mesh("african_head.obj", &mut image)?;
    }
   
    /*
    {
        let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
        let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
        let t2 = [Vec2::new(180, 150), Vec2::new(120, 160), Vec2::new(130, 180)];


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
