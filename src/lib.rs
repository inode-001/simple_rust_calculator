pub mod input_operation {
    use std::io;
    pub fn operators_input() -> char {
        loop {
            let mut operation = String::new();
            io::stdin()
                .read_line(&mut operation)
                .expect("Error when taking input");
            let operation = operation.trim();
            if operation.len() == 1 {
                return operation.chars().next().unwrap();
            } else {
                println!("Please type a valid operator (+, -, *, /) or 'q' to quit");
            }
        }
    }
    pub fn user_value_input() -> f64 {
        loop {
            let mut user_value = String::new();

            io::stdin()
                .read_line(&mut user_value)
                .expect("Error when reading the first value");
            match user_value.trim().parse::<f64>() {
                Ok(value) => return value,
                Err(_) => println!("Please type in a valid number : "),
            };
        }
    }
}
