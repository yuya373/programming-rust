use std::io::Write;

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn min<T: Ord>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

use std::fmt::Debug;
use std::hash::Hash;

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    use std::collections::HashMap;

    let mut store = HashMap::new();

    for v in values {
        if let Some(count) = store.get(v) {
            store.insert(v, count + 1);
        } else {
            store.insert(v, 0);
        }
    }

    // TODO
}

trait Vegetable {}
struct SingleVegeSalad<V: Vegetable> {
    veggies: Vec<V>,
}
struct CompileErrorSalad {
    // veggies: Vec<Vegetable>, // the size for values of type `(dyn Vegetable + 'static)` cannot be known at compilation time (doesn't have a size known at compile-time)
}
struct Salad {
    veggies: Vec<Box<Vegetable>>,
}

pub struct Sink;
use std::io::Result;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

trait StringSet {
    fn new() -> Self;
    fn from_slice(strings: &[&str]) -> Self;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    use std::fs::File;
    let mut local_file = File::create("/tmp/hello.txt")?;
    // conversion from &File to &Write (trait object).
    say_hello(&mut local_file);

    let mut bytes = vec![];
    say_hello(&mut bytes);
    assert_eq!(bytes, b"hello world\n");

    let mut buf: Vec<u8> = vec![];
    // let writer: Write = buf; // all local variables must have a statically known size
    let writer: &mut Write = &mut buf; // ok (trait object)

    // let w: Box<Write> = Box::new(local_file);

    say_hello_generic(&mut local_file);
    // say_hello_generic::<File>(&mut local_file);
    say_hello_generic(&mut bytes);
    // say_hello_generic::<Vec<u8>>(&mut bytes);

    let mut sink = std::io::sink();
    // Sink::write_all() do nothing
    // Sink::flush do nothing
    // rust may remove this function call in compile time
    say_hello_generic(&mut sink);

    assert_eq!('$'.is_emoji(), false);

    Ok(())
}
