struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}
#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" Ⅱ");

    (broom1, broom2)
}

// struct Bounds(usize, usize);
pub struct Bounds(pub usize, pub usize);

// only ASCII text
struct Acsii(Vec<u8>);

struct Onesuch;

#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            younger: Vec::new(),
            older: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.younger.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in slice {
        if i > greatest {
            greatest = i;
        }
        if i < least {
            least = i;
        }
    }

    Extrema { greatest, least }
}

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{Result, Write};

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    // log_devices: [std::fs::File; 8],
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn log(&self, message: &str) -> Result<()> {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message)
    }

    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_error(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

use std::rc::Rc;

struct Camera;
struct Accelerometer;

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    eyes: [Camera; 32],
    motion: Accelerometer,
}

fn main() {
    println!("Hello, world!");

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);

    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey Ⅱ");
    assert_eq!(hokey2.health, 100);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let o = Onesuch;

    let mut v = vec![];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);
    println!("pop: {:?}", v.pop());

    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    q.push(3);
    println!("q: {:?}", q);
    println!("pop: {:?}", q.pop());
    println!("q: {:?}", q);
    println!("pop: {:?}", q.pop());
    println!("q: {:?}", q);
    println!("pop: {:?}", q.pop());
    println!("q: {:?}", q);

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    use std::cell::RefCell;
    let ref_cell = RefCell::new("hello".to_string());

    let r = ref_cell.borrow();
    let count = r.len();
    assert_eq!(count, 5);

    // let mut w = ref_cell.borrow_mut(); // panic: already borrowed
    // w.push_str(" world");
}
