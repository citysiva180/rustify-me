// Check conditions and act on it

pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender says : What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Sorry, you have to leave!!")
    } else {
        println!("I will need your id to verify")
    }
    // Short
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age {}", is_of_age);
}
