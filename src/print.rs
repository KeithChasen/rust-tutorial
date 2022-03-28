pub fn run() {
    println!("Hello from print.rs");
    println!("Number: {}", 1);
    println!("My name is {} I'm {}", "Kostya", 33);

    // Positional Argument
    println!(
        "I am a {0}-developer and {1} can be used for {0}-development", 
        "web", "Rust"
    );

    // Named Arguments
    println!(
        "I am {name} and I learn {language}", 
        name = "Kostya", 
        language = "Rust"
    );

    // Placeholder traits
    println!(
        "Binary: {:b}, Hex: {:x} Octal {:o}", 
        10, 
        10,
        10
    );

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10)
}