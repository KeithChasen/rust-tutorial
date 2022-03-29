pub fn run() {
    //primitive, immutable fixed length string somewhere in memory
    let hello = "Hello";

    //Growable, heap-allocated data structure. Used weh it's needed to modify string
    let mut phrase = String::from("Hello there ");

    // get length
    println!("Length: {}", hello.len());

    // may add 1 char
    phrase.push('K');

    //add string
    phrase.push_str("ostya");

    //capacity in bytes
    println!("Capacity: {}", phrase.capacity());

    // is empty
    println!("Is empty: {}", phrase.is_empty());

    // contains substring
    println!("Contains 'Kostya': {}", phrase.contains("Kostya"));

    //replace
    println!("Replace: {}", phrase.replace("Kostya", "dude"));

    // loop through string
    for word in phrase.split_whitespace() {
        println!("{}", word);
    }

    println!("{} {}", hello, phrase);

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(3, s.len());

    assert_eq!(10, s.capacity());
    assert_eq!(11, s.capacity());

    println!("{}", s);
}