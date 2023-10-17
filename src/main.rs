use std::io::stdin;

#[derive(Debug)]
struct Guest {
    name: String,
    greeting: String,
}

impl Guest {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_owned(),
            greeting: greeting.to_owned(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Expected a string");

    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Guest::new("bert", "Yo bert welcome!"),
        Guest::new("asccii boi", "Yo what up 🔠"),
        Guest::new("bruce", "Aha yeah yeah"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list.iter().find(|guest| guest.name == name);

    match known_visitor {
        Some(guest) => guest.greet(),
        _ => println!("Not Allowed!"),
    }
}
