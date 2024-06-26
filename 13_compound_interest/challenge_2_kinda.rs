use std::io::{self, Write};

// One of the challenges for this project is to 
// work backwards and find what principal you would
// need assuming you want to reach a certain end amount

// This is too easy.

// This program will find the amount of time you need to sit 
// on your investment (for both compound and continuous) to get 
// to a certain end amount. Sounds fun, right?

fn main() {
    let principal: f32 = read_float("What is the principal? ".to_string());
    let interest: f32 = 
        read_float("What is the rate (%)? ".to_string()) * 0.01;
    // read as a percent, stored as a decimal
    let amount: f32 = read_float(
            "What is the desired end amount? ".to_string());
  
    let mut is_continuous_str = String::new();
    read_from_terminal(
        "Do you want to compound continuously (y/n)? ".to_string(),
        &mut is_continuous_str);

    let is_continuous: bool = 
        is_continuous_str.eq_ignore_ascii_case("y") ||
        is_continuous_str.eq_ignore_ascii_case("yes");

    if !is_continuous {
        let compound: i32 = read_int("How many times is the interest compounded? ".to_string());
        let years = calculate_compound_interest_time(
                        principal,
                        interest,
                        compound,
                        amount
                     );
        println!(
            "You need to invest ${:.2} for {:.1} years to get to ${:.2} when compounded at {:.1}% {} times annually.",
            principal, 
            years,
            amount,
            interest * 100.0,
            compound
        );

    } else {
        let years = calculate_continuous_interest_time(
                        principal,
                        interest,
                        amount);
        println!(
            "You need to invest ${:.2} for {:.1} years to get to ${:.2} compounded continuously at {:.1}%.",
            principal, 
            years, 
            amount,
            interest * 100.0
       );
    }
}

fn calculate_compound_interest_time(
    principal: f32, 
    interest: f32, 
    compound: i32,
    final_amount: f32,
) -> f32 {
    // To get this, I solved for t in A = P(1 + r/n)^nt
    // top     lnA - lnP
    //         ---------------  =  t 
    // bottom  n * ln(1 + r/n)
    
    let top = final_amount.ln() - principal.ln();
    let bottom =   (compound as f32)
                 * (1.0 + interest / (compound as f32)).ln();

    return top / bottom;

}

fn calculate_continuous_interest_time(
    principal: f32, 
    interest: f32, 
    final_amount: f32,
) -> f32 {
    // lnA - lnP = ln(e^rt)
    // lnA - lnP = rt
    // (lnA - lnP)/r = t
    (final_amount.ln() - principal.ln()) / interest
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
