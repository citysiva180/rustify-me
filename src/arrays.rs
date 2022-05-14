//Arrays  - Fixed list where elements are the same data types
use std::mem;
pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut numbers_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("Single Value {:?}", numbers[0]);
    //Re-Assign Values
    numbers_1[2] = 20;

    //Arrays are stack Allocated
    println!("Single Value: {}", numbers[0]);

    //Get Array Length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack Allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Arrays - Get Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
