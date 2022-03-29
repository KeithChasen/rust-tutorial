pub fn run() {
    let name = "Kostya";

    let mut age = 32;
    println!("Last year I was {}", age);

    age = 33;

    println!("My name is {}. I am {}", name, age);

    //constants
    const ID: i32 = 001;
    println!("ID : {}", ID);

    // assign multiple vars
    let (new_name, new_age) = ("Keith", 27);
    println!("{} is {}", new_name, new_age);
}