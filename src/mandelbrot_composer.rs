use crate::mandelbrot;
use crate::renderer;

#[derive(Debug)]
pub enum ColorCurve {
    Linear,
    Root(f64),
    Modulo(u32),
}

pub fn compose(
    plane: &mandelbrot::Plane,
    config: &mandelbrot::Config,
    curve: ColorCurve,
) -> renderer::Image {
    let (width, height) = config.get_size();
    let mut img = renderer::Image::new(width as usize, height as usize);

    for x in 0..width {
        for y in 0..height {
            let val = plane.get_bin(x, y);
            let val = translate_iterations(val, config, &curve);
            let pixel = renderer::Pixel::new(val, val, val);
            img.set_pixel(x, y, pixel);
        }
    }

    img
}

fn translate_iterations(val: Option<u32>, config: &mandelbrot::Config, curve: &ColorCurve) -> u8 {
    match curve {
        ColorCurve::Linear => translate_linear(val, config.max_iter),
        ColorCurve::Root(x) => translate_sqrt(val, *x, config.max_iter),
        ColorCurve::Modulo(n) => translate_modulo(val, *n),
    }
}

pub struct Layer<'a> {
    pub color_curve: ColorCurve,
    pub color_multiplier: (f64, f64, f64),
    pub weight: u32,
    pub plane: &'a mandelbrot::Plane,
    pub config: &'a mandelbrot::Config,
}

pub fn compose_layers(layers: Vec<Layer>) -> renderer::Image {
    let (width, height) = layers[0].config.get_size();
    let mut img = renderer::Image::new(width as usize, height as usize);

    let total_weight: u32 = layers.iter().map(|l| l.weight).sum();

    for x in 0..width {
        for y in 0..height {
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            for layer in &layers {
                let weight = layer.weight as f64 / total_weight as f64;
                let val = layer.plane.get_bin(x, y);
                let val = translate_iterations(val, layer.config, &layer.color_curve);
                r += (val as f64 * layer.color_multiplier.0 * weight) as u8;
                g += (val as f64 * layer.color_multiplier.1 * weight) as u8;
                b += (val as f64 * layer.color_multiplier.2 * weight) as u8;
            }
            let pixel = renderer::Pixel::new(r, g, b);
            img.set_pixel(x, y, pixel);
        }
    }

    img
}

pub fn translate_linear(val: Option<u32>, max_iter: u32) -> u8 {
    match val {
        Some(val) => {
            let norm = val as f64 / max_iter as f64;
            (norm * 255.0) as u8
        }
        None => 0,
    }
}

pub fn translate_sqrt(val: Option<u32>, x: f64, max_iter: u32) -> u8 {
    match val {
        Some(val) => {
            let norm = val as f64 / max_iter as f64;
            (norm.powf(1.0 / x) * 255.0) as u8
        }
        None => 0,
    }
}

pub fn translate_modulo(val: Option<u32>, n: u32) -> u8 {
    match val {
        Some(val) => {
            let mod_val = val % n;
            let norm = mod_val as f64 / n as f64;
            (norm * 255.0) as u8
        }
        None => 0,
    }
}
