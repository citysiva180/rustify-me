// Primitive str = Immutable fixed-length string somewhere in memory
// String  = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello"; //Primitive string
    let hello_siva = String::from("Hello Siva"); //Reference Type
    let mut string_functions = String::from("Hello ");
    //Get Length
    println!("Length {}", hello.len());
    println!("{}", hello_siva);
    string_functions.push('A'); //Appending a Character
    string_functions.push_str("World"); //Appending a String
    println!("Capacity:{}", hello_siva.capacity()); //Capacity in bytes
    println!("Is Empty: {}", hello_siva.is_empty()); //Check if Empty
    println!("Contains 'World' {}", hello_siva.contains("World")); //Checks if a string is present
    println!("Replace {}", hello.replace("World", "There"));
    println!("{}", hello);

    //Looping through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10); //String Capacity in Bytes
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity()); //Changing Capacity

    println!("{}", s);
}
