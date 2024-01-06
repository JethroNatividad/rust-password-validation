use std::io;
use std::io::Write;

// Program that has a hard coded password, takes in a password, and if password matches, show "Welcome!", else show "I don't know you!"

// Inputs: password
// Process: compare
// Outputs: if match: "Welcome!", else "I don't know you!"

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}


fn main() {
    // Set a list of users
    let password: String = "pass".to_string();
    // prompt for input_username: "What is the username?"
        // If not exist in users, ask again.
    // prompt for input_password: "What is the password?"
    let input_password: String = get_input("What is the password? ");
    // compare input_password and user password, if match
        // print "Welcome!"
    // if not match
        // print "I don't know you!"
    if password.eq(&input_password) {
        println!("Welcome");
    } else {
        println!("I don't know you!");
    }
    
}
