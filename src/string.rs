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

    println!("{} {}", hello, phrase);
}