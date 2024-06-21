use calculator_simple::input_operation;
fn main() {
    loop {
        println!(
            "Enter operation to execute : 
      '+' -> Addition
      '-' -> Subtraction
      '*'  -> Mutiplication
      '/' -> Division
      'q' -> Quit
"
        );
        let operator = input_operation::operators_input();

        if operator == 'q' {
            println!("QUITING...");
            break;
        }

        println!("Enter first value to perform operation on : ");
        let first_value = input_operation::user_value_input();

        println!("Enter second value to perform operation on : ");
        let second_value = input_operation::user_value_input();

        let result = match operator {
            '+' => first_value + second_value,
            '-' => first_value - second_value,
            '*' => first_value * second_value,
            '/' => {
                if second_value != 0.0 {
                    first_value / second_value
                } else {
                    println!("ZERO DIVISION ERROR: u can't divide a number by zero,try again");
                    continue;
                }
            }
            _ => {
                println!("Enter operator (+, -, *, /) or 'q' to quit:");
                continue;
            }
        };
        println!("The result is: {}", result);
    }
}
