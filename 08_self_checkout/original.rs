use std::io::{self, Write};

// 5.5% tax
const TAX: f32 = 0.055;

fn main() {
    let price1: f32 = read_float("Enter the price of item 1: ");
    let quantity1: i32 = read_int("Enter the quantity of item 1: ");
         
    let price2: f32 = read_float("Enter the price of item 2: ");
    let quantity2: i32 = read_int("Enter the quantity of item 2: ");
 
    let price3: f32 = read_float("Enter the price of item 3: ");
    let quantity3: i32 = read_int("Enter the quantity of item 3: ");

    let pairs: Vec<(i32, f32)> = vec![
        (quantity1, price1),
        (quantity2, price2),
        (quantity3, price3),
    ];
    
    let subtotal: f32 = pairs.iter().fold(0.0, |acc, &(x,y)| acc + (x as f32 * y));

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

fn read_float(prompt: &str) -> f32 {
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

fn read_int(prompt: &str) -> i32 {
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
