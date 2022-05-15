// Enums are types which have few definite values
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Right => println!("Avatar Moving Right"),
        Movement::Left => println!("Avatar Moving Left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
