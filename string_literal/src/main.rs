fn main() {
    let speech = "\"Ouch!\" said the well.";
    println!("{}", speech);

    let new_line = "In the room the women come and go,
Singing of Mount Abora";
    println!("{}", new_line);

    let no_new_line = "It was a bright, cold day in April, and \
                       there were four of us-\
                       more or less.";
    println!("{}", no_new_line);

    let raw_string = r"C:\Program Files\Grillas";
    println!("{}", raw_string);

    let raw_string_with_double_quote = r###"
This raw string started with 'r###"'
Therefore it does not end until we reach a quote mark('"')
followed immediately by three pound signs ('###'):
"###;
    println!("{}", raw_string_with_double_quote);

    // let method = b"GET";
    let method: &[u8; 3] = br###"GET"###;
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let _oodles = &noodles[1..];
    let poodles = "はろー";
    assert_eq!(poodles.len(), 9);
    assert_eq!(poodles.chars().count(), 3);

    // assert_eq!("ONE".to_lowercase(), "one");
    assert!("ONE".to_lowercase() == "one");
}
