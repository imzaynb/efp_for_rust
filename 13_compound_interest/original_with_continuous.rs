use std::io::{self, Write};

fn main() {
    let principal: f32 = read_float("What is the principal? ".to_string());
    let interest: f32 = 
        read_float("What is the rate (%)? ".to_string()) * 0.01;
    // read as a percent, stored as a decimal
    let years: i32 = read_int("How many years? ".to_string());
  
    let mut is_continuous_str = String::new();
    read_from_terminal(
        "Do you want to compound continuously (y/n)? ".to_string(),
        &mut is_continuous_str);

    let is_continuous: bool = 
        is_continuous_str.eq_ignore_ascii_case("y") ||
        is_continuous_str.eq_ignore_ascii_case("yes");

    if !is_continuous {
        let compound: i32 = read_int( "How many times is the interest compounded? ".to_string());
        let amount = calculate_compound_interest(
                        principal,
                        interest,
                        years,
                        compound
                     );
        println!(
            "${:.2} invested at {:.1}% for {} years compounded {} times a year is ${:.2}.",
            principal, interest * 100.0, years, compound, amount);
    } else {
        let amount = calculate_continuous_interest(
                        principal,
                        interest,
                        years);
        println!(
            "${:.2} invested at {:.1}% for {} years compounded continuously is ${:.2}.",
            principal, interest * 100.0, years, amount);
    }
}

fn calculate_compound_interest(
    principal: f32, 
    interest: f32, 
    years: i32,
    compound: i32
) -> f32 {
    // (1 + r/n)^(nt)
    let inside = (1.0 + interest / (compound as f32))
                    .powf((years * compound) as f32);
    // P*(1 + r/n)^(nt)
    principal * inside
}

fn calculate_continuous_interest(
    principal: f32, 
    interest: f32, 
    years: i32,
) -> f32 {
    // the exp function is a function that acts on a x:f32 and
    // returns the value e ^ x
    principal * (interest * (years as f32)).exp()
    // returns P * e^rt
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
