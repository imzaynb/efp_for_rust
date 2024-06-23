use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut quotes: HashMap<String, String> = HashMap::new();

    println!("Enter a whole bunch of quotes and authors. Type '-' to
stop.");
    
    loop {
        let mut quote = String::new();
        read_from_terminal("What is the quote? ", &mut quote);     

        let mut author = String::new();
        read_from_terminal("Who is the author? ", &mut author);     
        
        if quote.as_str() == "-" || author.as_str() == "-" {
            break;
        }
        quotes.insert(author, quote);
    }
    
    for (author, quote) in &quotes {
        println!("{author} says, \"{quote}\""); 
    }
}

fn read_from_terminal(prompt: &str, mut buffer: &mut String) {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output"); 
    
    io::stdin()
          .read_line(&mut buffer) 
          .expect("Whoops, couldn't read");
    
    *buffer = buffer.trim().to_string();
}
