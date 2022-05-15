use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("Args: {:?}", args);
    println!("Command Passed is {}", command);
    let name = "Siva";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, How are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not valid command");
    }
}
