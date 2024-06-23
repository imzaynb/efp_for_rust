use std::io::{self, Write};

const SQUARE_FEET_TO_SQUARE_METERS: f32 = 0.09290304;

fn main() {
    let length = read_float("What is the length of the room in feet? "); 
    let width = read_float("What is the width of the room in feet? "); 

    println!("You entered dimensions of {length} feet by {width} feet.");
    println!("The area is");
    println!("{} square feet", length * width);
    println!("{:.3} square meters", length * width * SQUARE_FEET_TO_SQUARE_METERS);
 
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
