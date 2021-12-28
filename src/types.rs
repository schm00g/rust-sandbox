/* 
    Primitive types--
    Integers: u8, i8, u16, i16...
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays
*/

// Rust is statically typed language
// But types are inferred when not stated

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545454;

    // Find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    println!("{:?}", (x, y, z, is_active));

    // https://youtu.be/zF34dRivLOw?t=1890
}