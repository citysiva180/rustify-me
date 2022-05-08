mod print;
mod string;
mod tuples;
mod types;
mod vars;

fn main() {
    // println!("Hello, world!"); //Feels like Javascript / Java
    print::run();
    vars::run();
    types::run();
    string::run();
    tuples::run();
}
