pub fn run() {
    //Print something to the console!
    println!("Who are you in this vast multiverse, Mr.Strange?");
    //Formatted String
    println!("{} Don't cast that spell! - {}", "Strange", "Wong");
    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Boston", "Code"
    );
    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "baseball"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex:{:x} Octal:{:0}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
