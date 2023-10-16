use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Expected a string");
    your_name
}

fn main() {
    println!("Hello, what's your name?");
    println!("What up my dude {}", what_is_your_name());
}
