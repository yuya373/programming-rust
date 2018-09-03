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
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}
