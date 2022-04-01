// Poorly Written Program | by MaybeAnonymous
use std::{io::{self, Write, stdout}, thread, time};
use crossterm::{
    execute,
    terminal::{self, ClearType}, cursor,
};
use colored::*;

// Grocery shop function, accepts `has_eggs`, `milk`, and `eggs`
fn grocery_shop(has_eggs: bool, milk: u32, eggs: u32) {
    for i in 0..10 {
        match i % 4 { // loading animation
            0 => print!("/"),  // frame 1
            1 => print!("-"),  // frame 2
            2 => print!("\\"), // frame 3
            3 => print!("|"),  // frame 4
            _ => print!("{}", "Error {i}".red().bold())
        }
        print!("\n"); // print the required `\n`
        thread::sleep(time::Duration::from_millis(200)); // wait for 200ms
        execute!(stdout(), cursor::MoveUp(1), terminal::Clear(ClearType::CurrentLine)).unwrap(); // clear current frame
        stdout().flush().unwrap();
    }
    execute!(stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap(); // clear animation
    stdout().flush().unwrap();
    thread::sleep(time::Duration::from_millis(500)); // wait 500ms
    match (milk, eggs, has_eggs) {
        // Has eggs
        (0,0,true) => println!("I have acquired nothing!"),
        (0,1,true) => println!("I have acquired {eggs} egg!"),
        (1,0,true) => println!("I have acquired a carton of milk!"),
        (1,1,true) => println!("I have acquired a carton of milk and an egg!"),
  
        (_,0,true) => println!("I have acquired {milk} cartons of milk!"),
        (0,_,true) => println!("I have acquired {eggs} eggs!"),
        (_,1,true) => println!("I have acquired an egg and {milk} cartons of milk!"),
        (1,_,true) => println!("I have acquired a carton of milk and {eggs} eggs!"),
   
        (_,_,true) => println!("I have acquired {milk} cartons of milk and {eggs} eggs!"),
        
        // No eggs
        (0,0,false) => println!("I have acquired nothing! Also, the store was missing some items."),
        (0,1,false) => println!("I have acquired nothing, as there were no eggs."),
        (1,0,false) => println!("I have acquired a carton of milk!"),
        (1,1,false) => println!("I have acquired a carton of milk, there were no eggs."),

        (_,0,false) => println!("I have acquired {milk} cartons of milk!"),
        (0,_,false) => println!("I have acquired no eggs, as there were no eggs."),
        (1,_,false) => println!("I have acquired a carton of milk, there were no eggs."),

        (_,_,false) => println!("I have acquired {milk} cartons of milk, there were no eggs."),
    }
}

fn main() { 
    let mut in_req_eggs = String::new();
    let mut in_req_milk = String::new();
    let mut in_egg_av = String::new();

    print!("{}", "How many milk cartons? ".bold());
    io::stdout().flush().unwrap();
    io::stdin() 
        .read_line(&mut in_req_milk)
        .expect("Failed to read amount of milk.");
     let req_milk: u32 = in_req_milk.trim().parse::<u32>().unwrap_or_else(|_|
                                                                         { println!("{}", "Choosing default milk value: 0".bright_blue());
                                                                           0 } 
                                                                        ); 
    print!("{}", "How many eggs? ".bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut in_req_eggs)
        .expect("Failed to read amount of eggs.");
    let req_eggs: u32 = in_req_eggs.trim().parse::<u32>().unwrap_or_else(|_|
                                                                         { println!("{}", "Choosing default eggs value: 0".bright_yellow()); 
                                                                           0 } 
                                                                        );
    print!("{}", "Should there be eggs available? ".bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut in_egg_av)
        .expect("Failed to read if there should be eggs available.");
    let egg_av: bool = match in_egg_av.to_lowercase().trim() {
                            "yes" | "yse" | "sey" | "sim" | "si" | "sure" | "true" | "absolutely" | "positive" | "1" | "y" => true,
                            "no" | "on" | "false" | "non" | "nao" | "não" | "nope" | "not" | "no!" | "negative" | "0" | "n" => false,
                            _ => {
                                println!("{}", "Eggs will be available.\n".red());
                                true
                            }
                        };

    grocery_shop(egg_av, req_milk, req_eggs);    
}
