use std::io::stdin;

#[derive(Debug)]
enum GuestAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Guest {
    name: String,
    action: GuestAction,
    age: i8,
}

impl Guest {
    fn new(name: &str, action: GuestAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            GuestAction::Accept => println!("Welcome to the tree house, {}", self.name),
            GuestAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house, {}", self.name);
                println!("{}", note);
                if self.age < 18 {
                    println!("Do not serve alcohol to {}", self.name)
                }
                println!("Note: {:}", note);
            }
            GuestAction::Probation => println!("{} is now a probationary member", self.name),
            GuestAction::Refuse => println!("Out!"),
        }
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
    let mut visitor_list = vec![
        Guest::new("Bert", GuestAction::Accept, 45),
        Guest::new(
            "Steve",
            GuestAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Guest::new("Fred", GuestAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty + enter to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|guest| guest.name == name);

        match known_visitor {
            Some(guest) => guest.greet(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Guest::new(&name, GuestAction::Probation, 0));
                }
            }
        }
    }
}
