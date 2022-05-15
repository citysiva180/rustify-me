mod arrays;
mod conditional;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod string;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    println!("Hello, world!"); //Feels like Javascript / Java
    print::run();
    vars::run();
    types::run();
    string::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditional::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
}
