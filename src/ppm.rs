#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }

    pub fn red() -> RGB {
        RGB { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> RGB {
        RGB { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> RGB {
        RGB { r: 0, g: 0, b: 255 }
    }

    pub fn black() -> RGB {
        RGB { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> RGB {
        RGB {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn grey(v: f32) -> RGB {
        let v = (v * 255.0) as u8;
        RGB {
            r: v,
            g: v,
            b: v,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<RGB>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            data.push(RGB::black());
        }
        Image {
            width,
            height,
            data,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: RGB) {
        self.data[((self.height - 1) - y) * self.width + x] = c;
    }

    pub fn get(&self, x: usize, y: usize) -> RGB {
        println!("x: {}, y: {}", x, y);
        self.data[((self.height - 1) - y) * self.width + x]
    }
}

impl From<&Image> for String {
    fn from(image: &Image) -> String {
        let mut buf = String::new();
        buf.push_str(format!("P3\n{} {}\n255\n", image.width, image.height).as_str());
        for color in image.data.iter() {
            buf.push_str(format!("{} {} {} ", color.r, color.g, color.b).as_str());
        }
        buf
    }
}
