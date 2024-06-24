use std::io::{self, Write};

// 5.5% tax
const TAX: f32 = 0.055;

// I already include the numeric-only safety as a result
// of copying code over from project to project. Moving
// straight into challenge 2!

fn main() {

    println!("Enter a negative number to quit. ");
    let mut count: i32 = 1;
    let mut pairs: Vec<(i32, f32)> = Vec::<(i32, f32)>::new();

    loop {
        let price: f32 = read_float(
                           format!("Enter the price of item {count}: "));
        let quantity: i32 = read_int(
                              format!("Enter the quantity of item {count}: "));
    
        if price < 0.0 || quantity < 0 {
            break;
        }
        
        pairs.push((quantity, price));
        count += 1;
    } 
    
    let subtotal: f32 = pairs.iter().fold(
                          0.0, 
                          |acc, &(x,y)| acc + (x as f32 * y)
                        );

    let tax: f32 = subtotal * TAX;
    let total: f32 = subtotal + tax;
    
    println!("Subtotal: ${subtotal:.2}");
    println!("Tax: ${tax:.2}");
    println!("Total: ${total:.2}");

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
