// Structs are used to create custom datatype | They are more like classes in OOPS

//Traditional Struct

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    //Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // println!("Color: {} {} {}", c.red, c.green, c.blue);
    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);
    let mut white = Person::new("Brad", "Traversy");
    println!("Hi, {} {}", white.first_name, white.last_name);

    let mut black = Person::new("Marlon", "Brando");
    println!("Hi, {}", black.full_name());

    let mut human = Person::new("Mary", "Condo");
    println!("Hi, {}", human.full_name());
    human.set_last_name("Williams");
    println!("Hi, {}", human.full_name());
    println!("Hi, {:?}", human.to_tuple());
}
