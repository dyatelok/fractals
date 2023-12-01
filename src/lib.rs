use itertools::*;
use num::complex::{Complex, Complex64};
use std::fmt;

pub struct Size {
    width: usize,
    height: usize,
    size: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            size: width * height,
        }
    }
}
pub struct Bounds {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Bounds {
    pub const fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

fn transform(size: &Size, bounds: &Bounds, pos: usize) -> (f64, f64) {
    let (x, y) = (pos % size.width, pos / size.width);
    let (x, y) = (
        (x as f64 + 0.5) / size.width as f64,
        (y as f64 + 0.5) / size.height as f64,
    );
    (
        bounds.x_min + x as f64 * (bounds.x_max - bounds.x_min),
        bounds.y_min + y as f64 * (bounds.y_max - bounds.y_min),
    )
}

fn iterations(mut z: Complex64, c: Complex64, max_iters: usize) -> usize {
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z.powu(2) + c;
    }
    max_iters
}

fn to_symbol(iters: usize) -> char {
    match iters {
        0..=2 => ' ',
        3..=5 => '.',
        6..=10 => '•',
        11..=30 => '*',
        31..=100 => '+',
        101..=200 => 'x',
        201..=400 => '$',
        401..=700 => '#',
        _ => '%',
    }
}

pub struct Mandelbrot {
    size: Size,
    bounds: Bounds,
    max_iterations: usize,
}

impl Mandelbrot {
    pub fn new(size: Size, bounds: Bounds, max_iterations: usize) -> Self {
        Self {
            size,
            bounds,
            max_iterations,
        }
    }
}

impl fmt::Display for Mandelbrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.size.size + self.size.height);
        (0..self.size.size)
            .chunks(self.size.width)
            .into_iter()
            .for_each(|chunk| {
                chunk
                    .map(|pos| transform(&self.size, &self.bounds, pos))
                    .map(|(x, y)| {
                        iterations(
                            Complex64::new(0.0, 0.0),
                            Complex64::new(x, y),
                            self.max_iterations,
                        )
                    })
                    .map(to_symbol)
                    .for_each(|ch| res.push(ch));
                res.push('\n');
            });

        write!(f, "{}", res)
    }
}

pub struct Julia {
    size: Size,
    bounds: Bounds,
    max_iterations: usize,
    c: Complex<f64>,
}

impl Julia {
    pub fn new(size: Size, bounds: Bounds, max_iterations: usize, c: Complex<f64>) -> Self {
        Self {
            size,
            bounds,
            max_iterations,
            c,
        }
    }
}

impl fmt::Display for Julia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.size.size + self.size.height);
        (0..self.size.size)
            .chunks(self.size.width)
            .into_iter()
            .for_each(|chunk| {
                chunk
                    .map(|pos| transform(&self.size, &self.bounds, pos))
                    .map(|(x, y)| iterations(Complex64::new(x, y), self.c, self.max_iterations))
                    .map(to_symbol)
                    .for_each(|ch| res.push(ch));
                res.push('\n');
            });

        write!(f, "{}", res)
    }
}

pub fn term_transform(w: u16, h: u16, font_frac_width_height: f32) -> (usize, usize) {
    let (mut w, mut h) = (w as f32, h as f32);
    if w <= font_frac_width_height * h {
        h = w as f32 / font_frac_width_height;
    } else {
        w = h as f32 * font_frac_width_height;
    }
    (w as usize, h as usize)
}
