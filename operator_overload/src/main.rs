use std::cmp::{Ordering, PartialOrd};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T, // exclusive
}

impl<T: PartialOrd + Debug> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        // println!("self: {:?} ", self);
        if self.lower == other.lower && self.upper == other.upper {
            return Some(Ordering::Equal);
        }
        if self.upper <= other.lower {
            return Some(Ordering::Less);
        }
        if self.lower >= other.upper {
            return Some(Ordering::Greater);
        }
        None
    }
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    type Output = [P];

    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    println!("Hello, world!");

    use std::ops::Add;
    assert_eq!(10.add(20), 30);

    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );

    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
    let left = Interval {
        lower: 10,
        upper: 30,
    };
    let right = Interval {
        lower: 20,
        upper: 40,
    };
    assert!(!(left < right));
    assert!(!(left >= right));

    let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
    desserts[0].push_str(" (fictional)");
    desserts[1].push_str(" (real)");

    use std::ops::IndexMut;
    (*desserts.index_mut(0)).push_str(" (fictional)");
    (*desserts.index_mut(1)).push_str(" (real)");
}
