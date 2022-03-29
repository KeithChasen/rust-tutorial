// vactors - resizible arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // get single val
    println!("{}", numbers[0]);

    // add item to vector
    numbers.push(15);

    //reassign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //remove last item
    numbers.pop();

    println!("{:?}", numbers);

    //get vector length
    println!("vector length: {}", numbers.len());

    //vectors are stack allocated
    println!("vector occupies: {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop through vectors
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Number Vec: {:?}", numbers);
}