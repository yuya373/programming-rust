use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) // (String, Vec<String)
        in table {
        println!("works by {}", artist);
            for work // String
                in works {
            println!("  {}", work);
        }
    }
}

fn show_ref(table: &Table) {
    for (artist, works) // (&String, &Vec<String>)
        in table {
        println!("works by {}", artist);
            for work // &String
                in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    // show(table); // table moved here.

    // assert_eq!(table["Gesualdo"][0], "many madrigals"); // use of moved value

    show_ref(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    sort_works(&mut table);
    show_ref(&table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort();

    let x = 10;
    let y = 20;
    let mut r = &x;

    if true {
        r = &y;
    }

    assert!(*r == 10 || *r == 20);

    struct Point {
        x: i32,
        y: i32,
    };

    let point: Point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // their referencing value are equal;
    assert!(!std::ptr::eq(rx, ry)); // but occupy different address

    let r: &usize = &factorial(6);
    assert_eq!(r + 1009, 1729);
    assert_eq!(r + &1009, 1729);

    let v = vec![1, 2, 3];
    // fat pointer: start address of slice and length
    let rv: &[i32] = &v[1..];

    // dangling pointer
    {
        // let r;
        {
            let x = 1;
            // r = &x; // brrowed value does not live long enough
        } // `x` dropped here while still borrowed
          // assert_eq!(*r, 1);
    } // borrowed value needs to live until here

    let v0 = 3;
    {
        let mut v = Vec::new();
        v.push(v0);
        {
            let _r = &v[0];
        }
    }

    static WORTH_POINTING_AT: i32 = 1000;
    let r: &'static i32 = &WORTH_POINTING_AT;
    f(r);

    let x = 10;
    g(&x);
    // f(&x); x does not live long enough

    let s: &i32;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        // s = smallest(&parabola); // `parabola` does not live long enough
    }

    struct S<'a> {
        r: &'a i32,
    }

    let s: &i32;
    {
        let x = 10;
        let s = S { r: &x };
    }

    struct T<'a> {
        s: S<'a>,
    }

    let x = 32;
    let s = S { r: &x };
    let _t = T { s: s };

    struct S2<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    let x = 10;
    let r: &i32;
    {
        let y = 20;
        {
            // x and y have same lifetime ('a).
            let s = S2 { x: &x, y: &y };
            // x and r have same lifetime
            // so, x and r and y have same lifetime
            // r = s.x;
        }
    } // y drop here, violate lifetime ('a)

    struct S3<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r: &i32;
    {
        let y = 20;
        {
            let s = S3 { x: &x, y: &y };
            r = s.x;
        }
    }

    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    // cannot move out of `v`
    // v is borrowed
    // let aside = v;

    r[0];

    let v = vec![4, 8, 19, 27, 34, 10];
    {
        // `r` borrow `v`
        let r = &v;
        r[0];
    } // r drop here
      // move!
    let aside = v;

    fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
        for elt in slice {
            vec.push(*elt);
        }
    }

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    // cannot borrow `wave` as immutable
    // because it is also borrowed as mutable
    // extend(&mut wave, &wave);

    {
        let mut_rw = &mut wave;
    }
    let rw = &wave;

    let mut x = 10;
    // immutable borrow
    let r1 = &x;
    // immutable borrow
    let r2 = &x;
    // cannot assign `x`
    // `x` is borrowed
    // x += 10;

    let mut y = 20;
    // mutable borrow
    let m1 = &mut y;
    // cannot borrow `y` as mutable more than once at a time
    // let m2 = &mut y;

    // cannot use `y`
    // `y` was mutably borrowed
    // let z = y;

    let mut w = (107, 109);
    // immutable reference
    let r = &w;
    // reborrowing immutable as immutable
    let r0 = &r.0;
    // cannot reborrow immutable as mutable
    // let m1 = &mut r.1;

    let mut v = (136, 139);
    let m = &mut v;
    // reborrowing mutable from mutable
    let m0 = &mut m.0;
    *m0 = 137;
    // reborrowing immutable from mutable
    let r1 = &m.1;
    // cannot use `v.1` because it was mutably borrowed
    v.1;
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

static mut STASH: &i32 = &128;
// fn f(p: &i32) {
// ↑ is equal to ↓
// fn f<'a>(p: &'a i32) {
// STASH is static, need static lifetime for `p`
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(p: &'a i32) {}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    // fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
    fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
        for i in 0..(self.elements.len()) {
            let el = &self.elements[i];
            if el.starts_with(prefix) {
                return Some(el);
            }
        }
        return None;
    }
}
