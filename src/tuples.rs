pub fn run() {
    //Working with tuples
    let person: (&str, &str, i8) = ("Brad", "Boston", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
