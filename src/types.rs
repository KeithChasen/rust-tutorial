pub fn run() {
    // deffault i32
    let x = 1;

    // default f64
    let y = 2.5;

    // add explicit type
    let a: i64 = 45454545454545;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
}