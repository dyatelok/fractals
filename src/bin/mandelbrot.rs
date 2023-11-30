use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;

const FPS: f32 = 10.0;
const SPF: f32 = 1.0 / FPS;

use num::complex::Complex;

struct Size {
    width: usize,
    height: usize,
    size: usize,
}

impl Size {
    const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            size: width * height,
        }
    }
}
struct Bounds {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Bounds {
    const fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

struct Mandelbrot {
    size: Size,
    bounds: Bounds,
    max_iterations: usize,
}

impl Mandelbrot {
    fn new(size: Size, bounds: Bounds, max_iterations: usize) -> Self {
        Self {
            size,
            bounds,
            max_iterations,
        }
    }
    fn transform(&self, pos: usize) -> (f64, f64) {
        let (x, y) = (pos % self.size.width, pos / self.size.width);
        let (x, y) = (
            (x as f64 + 0.5) / self.size.width as f64,
            (y as f64 + 0.5) / self.size.height as f64,
        );
        (
            self.bounds.x_min + x as f64 * (self.bounds.x_max - self.bounds.x_min),
            self.bounds.y_min + y as f64 * (self.bounds.y_max - self.bounds.y_min),
        )
    }
}

fn iterations(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);
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

use itertools::*;
use std::fmt;
use std::fmt::Display;

impl Display for Mandelbrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::with_capacity(self.size.size + self.size.height);
        for chunk in (0..self.size.size).chunks(self.size.width).into_iter() {
            let _ = chunk
                .map(|pos| self.transform(pos))
                .map(|(x, y)| iterations(x, y, self.max_iterations))
                .map(to_symbol)
                .map(|ch| res.push(ch))
                .collect::<()>();
            res.push('\n');
        }

        write!(f, "{}", res)
    }
}

use terminal_size::{terminal_size, Height, Width};

fn main() {
    let terminal_size = terminal_size();
    let Some((Width(init_w), Height(init_h))) = terminal_size else {
        panic!("Unable to get terminal size");
    };

    let (mut w, mut h) = (init_w as f32, init_h as f32);

    const FONT_FRAC_WIDTH_HEIGHT: f32 = 3.0;
    if w <= FONT_FRAC_WIDTH_HEIGHT * h {
        h = w as f32 / FONT_FRAC_WIDTH_HEIGHT;
    } else {
        w = h as f32 * FONT_FRAC_WIDTH_HEIGHT;
    }

    let (w, h) = (w as usize, h as usize);
    let h = h - 1 + h % 2;

    let size = Size::new(w, h);
    let bounds = Bounds::new(-2.0, 1.0, -1.0, 1.0);
    let set = Mandelbrot::new(size, bounds, 1000);

    print!("{}", set);
}
