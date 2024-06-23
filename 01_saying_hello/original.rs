use std::io::{self, Write};

fn main() {

    print!("What is your name? ");
    io::stdout().flush().expect("Failed to flush output"); 
    // flushing the output is necessary to print and recieve input
    // on the same line
   
    let mut input = String::new();
    // this has to be mutable so that the value of the variable can 
    // change.
    
    
    io::stdin()
          .read_line(&mut input) 
          .expect("Whoops, couldn't read");
    // the "&" is used to take a reference of a variable in the 
    // c++ sense. When you take a reference, you "borrow" the variable.
    // you can take infinite immutable borrows, but only 1 mutable one. 
        
    input = input.trim().to_string();
    // by default the read_line() call includes the \n, trim this.

    println!("Hello, {}, nice to meet you!", input); 
}
