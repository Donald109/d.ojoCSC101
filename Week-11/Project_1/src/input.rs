use std::io;

//gets user input and returns a string
pub fn get_input(msg: String) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    while input.trim().is_empty() {
        input = get_input("Please enter a valid input: ".to_string());
    }

    input
}
