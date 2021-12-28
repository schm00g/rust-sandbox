// Vectors - Resizable arrays (will have more use cases than arrays)

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Debug trait must be used to print out arrays/vectors "{:?}"

    println!("{:?}", numbers);

    // Get single val
    println!("Single Values: {}", numbers[0]);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // &numbers is a reference to numbers array

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vectors
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop & mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}