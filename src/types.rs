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

    //Boolean
    let is_active = true;
    
    let is_it: bool = false;

    // get boolean from expression
    let is_greater = 10 > 5;

    println!("{:?}", (x,y,a,is_active, is_it, is_greater))
}