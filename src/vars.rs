// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped Language

pub fn run() {
    let name = "Brad";
    let age = 37;
    println!("My Name is {} and I am {}", name, age);
    let mut city = "New York";
    println!("Brad Lives in {}", city);
    city = "Los Angeles";
    println!("Brad Lives in {}", city);

    //Defining const

    const ID: i32 = 001; //Note const should always have type defined
    println!("ID:{}", ID); //Defining typed variables

    //Assign Multiple Vars
    let (my_name, my_age) = ("Brad", 47);
    println!("{} is {}", my_name, my_age);
}
