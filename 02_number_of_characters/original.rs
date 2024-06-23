use std::io::{self, Write};

fn main() {
    print!("What is the input string? ");
    io::stdout().flush().expect("Failed to flush output"); 
    // flush to allow input on the same line   
 
    let mut input = String::new();

    io::stdin()
          .read_line(&mut input) 
          .expect("Whoops, couldn't read");

    input = input.trim().to_string();
    // trim the \n

    let size = input.len();
    
    println!("{input} has {size} characters.");
}
