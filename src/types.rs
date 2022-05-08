// Primitive Types
// Integer Types  :
/*
Integer Types
-------------
u8 - unsigned integer - no negative value with 8 bits
i8 -
u16 -
i16 -
u32 -
i32 - 32 bits?
u64 -
i64 - 64 bits?
u128 -
i123
Floats : f32, f64
-------
Boolean (bool)
Characters (char)
Tuples
Arrays


Rust is a statically types language, which means that it must know  the types of all variables at compile time,
however the compiler can usually infer what type we want to use based on the value and how we use it.


*/

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 1.2;

    // Add explicit type
    let z: i64 = 454545454545;

    // Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Setting up boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, z, is_active, is_greater));

    // Character
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
