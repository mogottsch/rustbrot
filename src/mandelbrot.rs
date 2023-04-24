use num_complex::Complex;
extern crate test;

#[derive(Debug)]
pub struct Config {
    width: u32,
    height: u32,
    pub max_iter: u32,
    i_min: f64,
    i_max: f64,
    r_min: f64,
    r_max: f64,
}

impl Config {
    pub fn build(
        width: u32,
        height: u32,
        max_iter: u32,
        i_min: f64,
        i_max: f64,
        r_min: f64,
        r_max: f64,
    ) -> Result<Config, &'static str> {
        if width < 1 || height < 1 {
            return Err("width and height must be greater than 0");
        }
        if max_iter < 1 {
            return Err("max_iter must be greater than 0");
        }
        if i_min >= i_max {
            return Err("i_min must be less than i_max");
        }
        if r_min >= r_max {
            return Err("r_min must be less than r_max");
        }
        Ok(Config {
            width,
            height,
            max_iter,
            i_min,
            i_max,
            r_min,
            r_max,
        })
    }

    fn bins_per_r_unit(&self) -> f64 {
        self.width as f64 / (self.r_max - self.r_min)
    }

    fn bins_per_i_unit(&self) -> f64 {
        self.height as f64 / (self.i_max - self.i_min)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn build_with_pos(
        width: u32,
        height: u32,
        max_iter: u32,
        center: Complex<f64>,
        zoom: f64,
    ) -> Config {
        let r_min = center.re - 1.0 / zoom;
        let r_max = center.re + 1.0 / zoom;
        let i_min = center.im - 1.0 / zoom;
        let i_max = center.im + 1.0 / zoom;
        Config {
            width,
            height,
            max_iter,
            i_min,
            i_max,
            r_min,
            r_max,
        }
    }
}

pub struct Plane {
    bins: Vec<Vec<Option<u32>>>,
}

impl Plane {
    fn new(width: u32, height: u32) -> Plane {
        Plane {
            bins: vec![vec![None; width as usize]; height as usize],
        }
    }

    fn apply(&mut self, callback: &dyn Fn(&mut Option<u32>, &u32, &u32)) {
        for x in 0..self.bins.len() {
            for y in 0..self.bins[x].len() {
                callback(
                    &mut self.bins[x as usize][y as usize],
                    &(x as u32),
                    &(y as u32),
                );
            }
        }
    }

    pub fn get_bin(&self, x: u32, y: u32) -> Option<u32> {
        self.bins[x as usize][y as usize]
    }
}

pub fn compute_plane(config: &Config) -> Plane {
    let mut plane = Plane::new(config.width, config.height);

    plane.apply(&|bin, x, y| {
        let c = translate_to_complex(*x, *y, config);
        let iter = iterate(&c, &config.max_iter);
        match iter {
            Some(i) => *bin = Some(i),
            None => *bin = None,
        }
    });
    plane
}

#[test]
fn test_plane() {
    let mut plane = Plane::new(10, 10);

    plane.apply(&|bin, _, _| {
        assert_eq!(bin, &None);
    });

    plane.apply(&|bin, x, y| {
        *bin = Some(*x + *y);
    });

    plane.apply(&|bin, x, y| {
        assert_eq!(bin.unwrap(), (*x + *y));
    });
}

fn translate_to_complex(x: u32, y: u32, config: &Config) -> Complex<f64> {
    let r = config.r_min + (x as f64 / config.bins_per_r_unit());
    let i = config.i_min + (y as f64 / config.bins_per_i_unit());

    Complex { re: r, im: i }
}

fn translate_to_bin(c: &Complex<f64>, config: &Config) -> (u32, u32) {
    let x = ((c.re - config.r_min) * config.bins_per_r_unit()) as u32;
    let y = ((c.im - config.i_min) * config.bins_per_i_unit()) as u32;
    (x, y)
}

#[test]
fn test_translate() {
    let c1 = Complex { re: 0.0, im: 0.0 };
    let config = Config::build(100, 100, 100, -1.0, 1.0, -1.5, 0.5).unwrap();

    let (x, y) = translate_to_bin(&c1, &config);
    assert_eq!(x, 75);
    assert_eq!(y, 50);
    let c2 = translate_to_complex(x, y, &config);
    assert_eq!(c1, c2);
}

fn iterate(c: &Complex<f64>, max_iter: &u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..*max_iter {
        z = z * z + c;
        // this naive check is faster than computing the norm
        if z.re > 2.0 || z.re < -2.0 || z.im > 2.0 || z.im < -2.0 {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_iterate() {
    let converge = [
        Complex { re: 0.0, im: 0.0 },
        Complex {
            re: -0.12,
            im: 0.758,
        },
    ];

    let diverge = [
        Complex { re: 1.0, im: 0.0 },
        Complex {
            re: -0.812,
            im: 0.371,
        },
        Complex {
            re: -0.812,
            im: -0.371,
        },
    ];

    converge.iter().for_each(|&c| {
        assert!(iterate(&c, &100).is_none());
    });

    diverge.iter().for_each(|&c| {
        assert!(iterate(&c, &100).is_some());
    });
}
