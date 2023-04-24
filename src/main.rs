#![feature(test)]

mod mandelbrot;
mod mandelbrot_composer;
mod renderer;

use num_complex::Complex;
use std::time::Instant;

fn main() {
    let config = mandelbrot::Config::build_with_pos(1000, 1000, 100, Complex::new(-0.5, 0.0), 1.0);
    let start = Instant::now();
    let plane = mandelbrot::compute_plane(&config);
    let duration = start.elapsed();
    println!("Plane computed in {:?}", duration);

    let layers = vec![
        mandelbrot_composer::Layer {
            color_multiplier: (1.0, 0.0, 0.0),
            weight: 3,
            plane: &plane,
            config: &config,
            color_curve: mandelbrot_composer::ColorCurve::Linear,
        },
        mandelbrot_composer::Layer {
            color_multiplier: (0.0, 1.0, 0.0),
            weight: 3,
            plane: &plane,
            config: &config,
            color_curve: mandelbrot_composer::ColorCurve::Root(3.0),
        },
        mandelbrot_composer::Layer {
            color_multiplier: (0.0, 0.0, 1.0),
            weight: 1,
            plane: &plane,
            config: &config,
            color_curve: mandelbrot_composer::ColorCurve::Modulo(10),
        },
    ];

    let img = mandelbrot_composer::compose_layers(layers);
    // let img =
    //     mandelbrot_composer::compose(&plane, &config, mandelbrot_composer::ColorCurve::Modulo(10));

    img.render("test.png");
}
