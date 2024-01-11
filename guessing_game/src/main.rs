use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Please enter a message");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        
        let guess = get_number();
        println!("You wrote: {}, secret was: {secret_number}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You WON!");
                exit(0);
            },
        }
    }
}

fn get_number() -> u32 {
    loop {
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Issue reading the input");

        let input: u32 = match input
            .trim()
            .parse() {
                Ok(num) => return num,
                Err(_) => {
                    println!("Your input was incorrect, please use numbers");
                    continue;
                }
            };


    };
}
