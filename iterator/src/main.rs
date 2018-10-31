fn main() {
    println!("Hello, world!");

    fn triangle(n: i32) -> i32 {
        let mut sum = 0;

        for i in 1..n + 1 {
            sum += i;
        }

        sum
    }

    fn triangle_fold(n: i32) -> i32 {
        (1..n + 1).fold(0, |acc, n| acc + n)
    }

    println!("triangle(9): {}", triangle(9));
    println!("triangle_fold(9): {}", triangle_fold(9));

    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }
    // rust rewrite for loop
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));

    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());

    let mut iterator = favorites.into_iter();
    assert_eq!(iterator.next(), Some("Liebestraume No. 3".to_string()));
    assert_eq!(
        iterator.next(),
        Some("Lucy in the Sky With Diamonds".to_string())
    );
    assert_eq!(iterator.next(), None);

    let mut v = vec!["mutable".to_string(), "iterator".to_string()];

    for s in &mut v {
        s.push_str(" a!");
    }

    println!("v: {:?}", v);

    let mut it = (&mut v).into_iter();
    while let Some(s) = it.next() {
        s.push_str(" aa!");
    }

    println!("v: {:?}", v);

    let _it = v.into_iter(); // move occurs here

    // let _it = v.into_iter(); // use of moved value `v`

    use std::fmt::Debug;
    // generic `dump` function
    fn dump<T, U>(t: T)
    where
        // restrict `t` with itrable and debuggable item
        T: IntoIterator<Item = U>,
        U: Debug,
    {
        for u in t {
            println!("{:?}", u);
        }
    }

    dump(vec![1, 2, 3, 7]);
    dump(vec!["foo", "bar"]);
    struct NonDebugStruct;
    // dump(vec![NonDebugStruct {}]); // Debug is not implemented for NonDebugStruct

    use std::iter::FromIterator;
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    let text = "  ponies  \n   giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    let text = "  ponies  \n   giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|&s| s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    // iterator adaptors are lazy and do nothing unless consumed
    ["earth", "water", "air", "fire"]
        .iter()
        .map(|e| println!("{}", e));

    // consume by `collect()`
    ["earth", "water", "air", "fire"]
        .iter()
        .map(|e| println!("{}", e))
        .collect::<Vec<()>>();

    use std::str::FromStr;
    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{}", number);
    }

    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    let iter = (0..10).scan(0, |acc, e| {
        *acc += e;
        if *acc > 10 {
            None
        } else {
            Some(e * e)
        }
    });

    assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 4, 9, 16]);

    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get aaa\r\n\
                   bbbb";

    let lines = message.lines();

    let iter = lines.take_while(|l| !l.is_empty());
    for header in iter {
        println!("{}", header);
    }

    let iter = message.lines().skip_while(|l| !l.is_empty()).skip(1);
    for body in iter {
        println!("{}", body);
    }

    use std::iter::Peekable;

    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n,
            }
            tokens.next();
        }
    }

    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);

    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    use std::iter::DoubleEndedIterator;

    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();
    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);

    let upper_case: String = "aあb"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after:    {:?}", c))
        .collect();

    assert_eq!(upper_case, "AあB");

    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);

    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::repeat;

    let endings = vec!["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice")
        ]
    );

    let message = "To: jimb\r\n\
                   From: id\r\n\
                   \r\n\
                   Oooooh, donuts!!\r\n";

    let mut lines = message.lines();

    println!("Headers:");
    let headers = lines.by_ref().take_while(|l| !l.is_empty());
    for header in headers {
        println!("{}", header);
    }

    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    let a = ['1', '2', '3'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));

    use std::iter::once;

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tp| match tp {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }

    // use std::io::prelude::*;

    // let stdin = std::io::stdin();
    // println!("{}", stdin.lock().lines().count());
}
