fn main() {
    struct Person {
        name: String,

        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    let x = vec![10, 20, 30];
    let c = true;

    if c {
        f(x);
    } else {
        g(x);
    }
    // h(x); // x is uninitialized if either path uses it

    let x = vec![10, 20, 30];
    while _f() {
        // g(x); // x would be moved in first iteration,
        // uninitialized in second iteration
    }

    let mut x = vec![10, 20, 30];

    while _f() {
        g(x); // move from x
        x = _h(); // give x a fresh value
    }

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let third = v[2]; // cannot move out indexed content

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    println!("vec: {:?}", v);

    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    println!("vec: {:?}", v);

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    println!("vec: {:?}", v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    println!("vec: {:?}", v);

    let v = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    struct _Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(_Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });

    // let first_name = composers[0].name; // cannot move out indexed contents
    // println!("first name: {:?}", first_name);

    // let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

    let l = Label { number: 3 };
    print(l);
    print(l);

    use std::rc::Rc;

    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
// #[derive(Copy, Clone)] // String does not impl Copy
struct StringLabel {
    name: String,
}
#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}
fn f(_v: Vec<i32>) {}
fn g(_v: Vec<i32>) {}
fn h(_v: Vec<i32>) {}

fn _f() -> bool {
    false
}

fn _h() -> Vec<i32> {
    vec![1, 2, 3]
}
