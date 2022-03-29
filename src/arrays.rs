// arrays: fixed length and same data type

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // get single val
    println!("{}", numbers[0]);

    //reassign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //get array length
    println!("array length: {}", numbers.len());

    //arrays are stack allocated
    println!("array occupies: {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}