use image::ImageBuffer;

#[derive(Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_image_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([self.r, self.g, self.b])
    }
}

pub struct Image {
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Pixel::new(0, 0, 0));
            }
            pixels.push(row);
        }
        Self { pixels }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.pixels[y as usize][x as usize] = pixel;
    }

    pub fn render(&self, filename: &str) {
        let img = ImageBuffer::from_fn(
            self.pixels.len() as u32,
            self.pixels[0].len() as u32,
            |x, y| self.pixels[y as usize][x as usize].to_image_rgb(),
        );
        img.save(filename).unwrap();
    }
}
