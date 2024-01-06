extern crate bcrypt;

use std::io;
use std::io::Write;
use bcrypt::{verify};

// Program that has a list of users, takes in a username and password, and if username and password matches, show "Welcome, Username!", else show "Incorrect password!"

// Inputs: username, password
// Process: find username in vector, if exist compare password with bcrypt verify.
// Outputs: if match: "Welcome, Username!", else "Incorrect password!"

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

struct User {
    username: String,
    password: String
}

fn main() {
    // Set a list of users
    let users: Vec<User> = vec![
        User {
            username: "arthur".to_string(),
            password: "$2a$12$NkAu.TNtuW7tRejzKPKduO1QxhkwLmAyizXnflTGRPtiFlKXjm9qK".to_string()
        },
        User {
            username: "john".to_string(),
            password: "$2a$12$BArgAwz74L2n8ziSctOjwuVLxCmw4LwWO8ZkRJWDoMKrPpbn0P61q".to_string()
        }
    ];
    // prompt for input_username: "What is the username?"
    let input_username: String = get_input("What is the username? ");
        // If not exist in users, ask again.
    let username_exists = users.iter().find(|user| user.username == input_username);

    if let Some(user) = username_exists {
        // prompt for input_password: "What is the password?"
        let input_password: String = get_input("What is the password? ");
        // compare input_password and user password, if match
            // print "Welcome, Username!"
        // if not match
            // print "Incorrect password!"
        if verify(&input_password, &user.password).unwrap() {
            println!("Welcome, {}!", user.username);
        } else {
            println!("Incorrect password!");
        }
    } else {
        println!("Cannot find Username!");
    }

}
