use backend::{logs::LogsModule, notes::NotesModule, BasilModule};
use std::io;

fn main() {
    let mut modules: Vec<Box<dyn BasilModule>> = Vec::new();

    modules.push(Box::new(NotesModule {}));
    modules.push(Box::new(LogsModule {}));

    loop {
        // Create a mutable string to store user input
        let mut user_input = String::new();

        // Prompt the user for input
        println!("Enter something");

        // Read user input from the standard input (keyboard)
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Trim leading and trailing whitespaces
        let mut user_input = user_input.trim();

        // Determine appropriate module for request
        for module in modules.iter() {
            let keywords: Vec<String> = module.get_keywords();

            if module_keyword_match(
                &user_input.split_whitespace().map(String::from).collect(),
                &keywords,
            ) {
                loop {
                    match module.run(user_input.to_string()) {
                        Ok(response) => {
                            if response.is_finished {
                                println!("{}", response.text_response);
                                break;
                            }
                            println!("{}", response.text_response);
                            let mut new_input = String::new();
                            io::stdin()
                                .read_line(&mut new_input)
                                .expect("Failed to read line...");

                            user_input = user_input.trim()
                        }
                        Err(error) => {
                            println!("{}", error.error_text);
                            break;
                        }
                    };
                }
                break;
            }
        }
    }
}

pub fn module_keyword_match(user_input: &Vec<String>, keywords: &Vec<String>) -> bool {
    for user_word in user_input {
        for keyword in keywords {
            if keyword.to_lowercase() == user_word.to_lowercase() {
                return true;
            }
        }
    }

    false
}
