use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(Debug)]
struct Rect {
    width: u64,
    height: u64
}

impl Rect {

    fn square(size: u64) -> Rect {
        Rect { width: size, height: size }
    }

    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn fits_in(&self, other: &Rect) -> bool {
        self.width <= other.width && self.height <= other.height
    }

    fn transpose(&self) -> Rect {
        Rect { width: self.height, height: self.width }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]: {}", self.height, self.width, self.area())
    }
}

fn main() {

    let rects: [Rect; 3] = [
        Rect { width: 50, height: 180 },
        Rect { width: 120, height: 45 },
        Rect::square(45),
    ];

    rects.iter()
        .permutations(2)
        .for_each(|p| print_rects_comparison(p[0], p[1]));
}

fn print_rects_comparison(r1: &Rect, r2: &Rect) {

    println!("\n");
    if r1.fits_in(&r2) {
        println!("Rect {} fits in rect {}", r1, r2);
    } else if r1.transpose().fits_in(&r2) {
        println!("Transposed rect {} fits in rect {}", r1.transpose(), r2);
    } else {
        println!("Rect {} cannot fit in rect {}", r1, r2);
    }
}