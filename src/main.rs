mod arrays;
mod conditional;
mod functions;
mod loops;
mod print;
mod string;
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
}
