// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Debug trait must be used to print out arrays "{:?}"

    println!("{:?}", numbers);

    // Get single val
    println!("Single Values: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // &numbers is a reference to numbers array

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}