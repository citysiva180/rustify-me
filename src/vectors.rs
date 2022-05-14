//Arrays  - Fixed list where elements are the same data types
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
    println!("Single Value {:?}", numbers[0]);
    //Re-Assign Values
    numbers[2] = 20;

    //Vector are stack Allocated
    println!("Vector Value: {}", numbers[0]);

    //Get Vector Length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack Allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Vector - Get Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Adding elements to vectors
    numbers.push(5);
    numbers.push(6);
    println!("Before pop: {:?}", numbers);
    numbers.pop();
    println!("After pop: {:?}", numbers);

    //Looping is also possible
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and Mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec:{:?}", numbers);
}
