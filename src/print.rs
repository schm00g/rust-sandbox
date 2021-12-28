pub fn run(){
    // print to console
    println!("Printer go brrrrr!!");

    // to print an integer
    println!("Number: {}", 1);

    // basic formatting
    println!("{} is from {}", "Brad", "Mass");

    // positional args
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // named args
    println!("{name} likes to play {activity}", name = "John", activity = "Football");

    // placeholder traits 
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait - tuple
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10); 
}