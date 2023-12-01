use fractals::{term_transform, Bounds, Julia, Size};
use num::complex::Complex64;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let terminal_size = terminal_size();
    let Some((Width(w), Height(h))) = terminal_size else {
        panic!("Unable to get terminal size");
    };

    const FONT_FRAC_WIDTH_HEIGHT: f32 = 3.0;
    let (w, h) = term_transform(w, h, FONT_FRAC_WIDTH_HEIGHT);
    let h = h - 1 + h % 2;

    let size = Size::new(w, h);
    let bounds = Bounds::new(-1.5, 1.5, -1.5, 1.5);
    let set = Julia::new(size, bounds, 1000, Complex64::new(0.0, 0.7));

    print!("{}", set);
}
