// Loops - iterate until a condition is met

pub fn run(){
    let mut count = 0;
    let mut number = 0;

    loop {
        count += 1;
        println!("Number: {}", count);

        // terminating condition
        if count == 20 {
            break;
        }
    }

    // Fizz Buzz
    loop {
        count += 1;

        if count % 3 == 0 {
            println!("Number is {}: {}", count, "Fizz")
        }

        if count % 5 == 0 {
            println!("Number is {}: {}", count, "Buzz");
        }

        if count == 100 {
            break;
        }
    }

    // While Fizz/Buzz
    while number <= 100 {
        if number % 15 == 0 {
            println!("Fizzbuzz")
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
        number += 1;
    }
}