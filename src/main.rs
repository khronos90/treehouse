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
    let mut visitor_list = vec![
        Guest::new("bert", "Yo bert welcome!"),
        Guest::new("asccii", "Yo what up 🔠"),
        Guest::new("bruce", "Aha yeah yeah"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty + enter to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|guest| guest.name == name);

        match known_visitor {
            Some(guest) => guest.greet(),
            None => {
                if name.is_empty() {
                    println!("Final list of visitors: {:#?}", visitor_list);
                    break;
                } else {
                    println!("{:?} is now in the guest list", name);
                    visitor_list.push(Guest::new(&name, "New friend"));
                }
            }
        }
    }
}
