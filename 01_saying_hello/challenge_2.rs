use std::io::{self, Write};

// challenge one is impossible because the read_line()
// call writes into a seperate variable, in this case "input".
// could this be done in a closure? 

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Failed to flush output"); 
   
    let mut input = String::new();
    
    io::stdin()
          .read_line(&mut input) 
          .expect("Whoops, couldn't read");
        
    input = input.trim().to_string();

    let greeting = match input.to_lowercase().as_str() {
        "zayn" => format!("Welcome back, {}!", input),
        "joe"  => format!("Go away, {}!", input),
        _ => format!("Hello, {}. Nice to meet you!", input),
    };

    println!("{}", greeting); 
}
