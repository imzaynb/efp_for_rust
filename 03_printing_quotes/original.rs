use std::io::{self, Write};

fn read_from_terminal(prompt: &str, mut buffer: &mut String) {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output"); 
    
    io::stdin()
          .read_line(&mut buffer) 
          .expect("Whoops, couldn't read");
    
    *buffer = buffer.trim().to_string();
}

fn main() {
    let mut quote = String::new();
    read_from_terminal("What is the quote? ", &mut quote);     

    let mut author = String::new();
    read_from_terminal("Who is the author? ", &mut author);     

    println!("{author} says, \"{quote}\""); 
}
