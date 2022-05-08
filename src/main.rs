mod print;
mod types;
mod vars;

fn main() {
    // println!("Hello, world!"); //Feels like Javascript / Java
    print::run();
    vars::run();
    types::run();
}
