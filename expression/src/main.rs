fn main() {
    let strings: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
    // each String is moved into s here
    for s in strings {
        println!("{}", s);
    } // and dropped here
      // println!("strings: {}", strings.len()); // use of moved value

    let strings: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
    for rs in &strings {
        println!("String: {:?} is at address {:p}", *rs, rs);
    }
    println!("strings: {}", strings.len());

    let mut strings: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
    for rs in &mut strings {
        rs.push('\n');
    }
    println!("strings: {:?}", strings);

    'foo: loop {
        'bar: loop {
            println!("loop bar");
            break 'bar;
        }
        println!("loop foo");
        break 'foo;
    }

    struct Player {}
    impl Player {
        fn new() -> Player {
            Player {}
        }

        fn location(&self) -> (f64, f64) {
            (1.0, 0.0)
        }
    }
    let player = Player::new();
    player.location();

    #[derive(Debug)]
    struct Game {
        black_pawns: u64,
    }

    let mut game = Game { black_pawns: 0 };
    game.black_pawns = 0x00ff0000_000000000_u64;
    println!("Game: {:?}", game);

    let mut coords = (3.3, 3.3);
    coords.1 = 0.0;
    println!("coords: {:?}", coords);

    let padovan: Vec<u64> = vec![8, 88, 888];
    // elem: &u64
    // *elem: u64
    for elem in &padovan {
        println!("v: {}", *elem);
    }

    let is_even = |x| -> bool { x % 2 == 0 };
    assert_eq!(is_even(14), true);
}

fn no_return() -> ! {
    loop {}
}
