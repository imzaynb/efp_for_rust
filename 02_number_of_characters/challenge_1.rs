use std::io::{self, Write};

fn main() {
    let mut input = String::new();
   
    loop { 
        print!("What is the input string? ");
        io::stdout().flush().expect("Failed to flush output"); 
        
        io::stdin()
              .read_line(&mut input) 
              .expect("Whoops, couldn't read");

        input = input.trim().to_string();
        
        let size = input.len();
    
        match size {
            0 => println!("You must enter something."),
            _ => {
                println!("{input} has {size} characters.");
                return;
            },
        }
    }
}
