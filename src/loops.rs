pub fn run() {
    let mut count = 0;
    // infinity~
    loop {
        count += 1;
        println!("Number: {}", count);
        if count == 20 {
            break;
        }
    }

    let mut count_1 = 0;
    // While Loop (FizzBuzz)
    while count_1 <= 100 {
        if count_1 % 15 == 0 {
            println!("{:?} - FizzBuzz", count_1);
        } else if count_1 % 3 == 0 {
            println!("{:?} - Fizz", count_1);
        } else if count_1 % 5 == 0 {
            println!("{:?} - Buzz", count_1);
        } else {
            println!("{:?}", count_1)
        }
        //Increment
        count_1 = count_1 + 1;
    }

    //For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("KnickKnack")
        } else if x % 3 == 0 {
            println!("Knick")
        } else if x % 5 == 0 {
            println!("Knack")
        } else {
            println!("{}", x)
        }
    }
}
