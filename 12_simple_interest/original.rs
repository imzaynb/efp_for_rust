use std::io::{self, Write};

fn main() {
    let principal: f32 = read_float("Enter the principal: ".to_string());
    let interest: f32 = 
        read_float("Enter the rate of interest (%): ".to_string()) * 0.01;
    // read as a percent, stored as a decimal
    let years: i32 = read_int("Enter the number of years: ".to_string());

    // A = P(1 + rt)
    let amount = principal * (1.0 + (years as f32 * interest));

    println!("After {:.2} years at {:.2}%, the investment will be worth ${:.2}",
                years, interest * 100.0, amount);
}

fn read_from_terminal(prompt: String, mut buffer: &mut String) {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output"); 
    
    io::stdin()
          .read_line(&mut buffer) 
          .expect("Whoops, couldn't read");
    
    *buffer = buffer.trim().to_string();
}

fn read_float(prompt: String) -> f32 {
    let mut float_str = String::new(); 
    let float: f32;

    loop {
        read_from_terminal(format!("{prompt}"), &mut float_str); 
        match float_str.parse() {
            Ok(number) => {
                float = number;
                break;
            },
            Err(_) => {
                println!("Please only input a number");
                float_str.clear();
            }
        }
    }

    float
}

fn read_int(prompt: String) -> i32 {
    let mut int_str = String::new(); 
    let int: i32;

    loop {
        read_from_terminal(format!("{prompt}"), &mut int_str); 
        match int_str.parse::<f32>() {
            Ok(number) => {
                if number.fract() == 0.0 {
                    int = number as i32;
                    break;
                } else {
                    println!("Please enter a non-fractional number");
                    int_str.clear();
                }
            },
            Err(_) => {
                println!("Please only input a number");
                int_str.clear();
            }
        }
    }

    int
}
