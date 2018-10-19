fn main() {
    println!("Hello, world!");

    struct Appellation {
        name: String,
        nicknames: Vec<String>,
    }

    impl Drop for Appellation {
        fn drop(&mut self) {
            print!("Dropping: {}", self.name);
            if !self.nicknames.is_empty() {
                print!(" (AKA {})", self.nicknames.join(", "));
            }
            println!("");
        }
    }

    {
        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };

        println!("before assignment");
        a = Appellation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };
        println!("at end of block");
    }

    let p;
    {
        let q = Appellation {
            name: "Cardamine hirsuta".to_string(),
            nicknames: vec!["shotweed".to_string(), "bittercress".to_string()],
        };
        if false {
            p = q;
            println!("at end of if block")
        }

        println!("at end of block");
    }

    println!("at end of main block");

    struct S<T: ?Sized> {
        b: Box<T>,
    }

    let p: S<str>; // unsized (slice)
    let p: S<std::io::Write>; // unsized (trait object)
    let p: S<i32>; // sized
    let p: S<String>; // sized

    struct RcBox<T: ?Sized> {
        ref_count: usize,
        value: T,
    }

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };
    use std::fmt::Display;
    let boxed_displayable: &RcBox<Display> = &boxed_lunch;

    fn display(boxed: &RcBox<Display>) {
        println!("For your enjoyment: {}", &boxed.value);
    }

    display(&boxed_lunch);
    display(&boxed_displayable);

    struct Selector<T> {
        elements: Vec<T>,
        current: usize,
    }

    use std::ops::{Deref, DerefMut};

    impl<T> Deref for Selector<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.elements[self.current]
        }
    }

    impl<T> DerefMut for Selector<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.elements[self.current]
        }
    }

    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z'); // Deref trait
    assert!(s.is_alphabetic()); // Deref trait

    *s = 'w'; // DerefMut trait?

    assert_eq!(*s, 'w'); // Deref Trait
    assert_eq!(s.elements, ['x', 'y', 'w']);

    fn show_it(thing: &str) {
        println!("{}", thing);
    }

    let s = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2,
    };

    show_it(&s);

    fn show_it_generic<T: Display>(thing: T) {
        println!("{}", thing);
    }
    // show_it_generic(&s); // Display is not implemented for Selector
    show_it_generic(&s as &str);

    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    use std::collections::HashMap;
    let map: HashMap<String, i32> = HashMap::new();

    use std::hash::Hash;
    fn get<'a, K, V>(map: &'a HashMap<K, V>, key: &K) -> Option<&'a V>
    where
        K: Eq + Hash,
    {
        map.get(key)
    }

    // use std::borrow::Borrow;
    // fn get_2<'a, K, V, Q: ?Sized>(map: &'a HashMap<K, V>, key: &Q) -> Option<&'a V>
    // where
    //     K: Borrow<Q>,
    //     Q: Eq + Hash,
    // {
    //     map.get(key)
    // }

    use std::net::Ipv4Addr;
    fn ping<A>(address: A) -> std::io::Result<bool>
    where
        A: Into<Ipv4Addr>,
    {
        let ipv4_address = address.into();
        Ok(true)
    }

    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141)));
    println!("{:?}", ping([66, 146, 219, 98]));
    println!("{:?}", ping(0xd0763b94_u32));

    let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
    let addr2 = Ipv4Addr::from(0xd0763b94_u32);

    let text = "Beautiful Soup".to_string();
    let bytes: Vec<u8> = text.into();
    println!("Beautiful Soup: {:?}", bytes);
    // println!("text is moved: {:?}", text);

    use std::borrow::Cow;
    use std::path::PathBuf;

    enum Error {
        OutOfMemory,
        StackOverflow,
        MachineOnFire,
        Unfathomable,
        FileNotFound(PathBuf),
    }

    fn describe(error: &Error) -> Cow<'static, str> {
        match *error {
            Error::OutOfMemory => "out of memory".into(),
            Error::StackOverflow => "stack overflow".into(),
            Error::MachineOnFire => "machine on fire".into(),
            Error::Unfathomable => "machine bewildered".into(),
            Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
        }
    }

    let error = Error::OutOfMemory;
    let desc = describe(&error);
    println!("Disaster has struck: {}", desc); // borrow
    let desc_owned = desc.into_owned(); // own
}
