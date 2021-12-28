// variables hold primitive data of refences to data
// variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 123;
    println!("My name is {} and I am {}", name, age);
    age = 111;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( my_name, my_age ) = ("Brad", 32);
    println!("{} is {}", my_name, my_age);
}