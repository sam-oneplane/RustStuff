use std::io;
use std::cmp::Ordering;
use rand::{Rng};
use colored::*;

pub struct Guess 
{
    value: u32, 
}
impl Guess 
{
    pub fn new(value: u32) -> Guess
    {
        if value < 1 || value > 100 
        {
            panic!("guess value should be between 1 and 100, got {}", value);
        }
        Guess { value : value }    
    }
    pub fn value(&self) -> u32 
    {
        self.value
    }
}

fn main() 
{
    println!("generating random number 1 to 100!");
    let rnd_number = rand::thread_rng().gen_range(1 .. 101); /* can not use const because rand calc. at run time */
    loop 
    {
        println!("please guess a number...");
        let mut guess = String::new();
        /* read string line from stdin */
        io::stdin()
            .read_line(& mut guess)
            .expect("failed to read line");
        /* guess the value using match : */
        let guess: u32 = match guess.trim().parse()
        {
            /*
                enum Result<T, E> {
                    Ok(T),
                    Err(E),
                }
             */
            Ok(num) => num,
            Err(_) => continue , // if no number string
        };
        let guess_in = Guess::new(guess);
        println!("you guessed : {}", guess_in.value());

        match guess_in.value().cmp(&rnd_number) 
        {
            Ordering::Less => println!("{}", "TO SMALL".red()),
            Ordering::Greater => println!("{}", "TO BIG".red()),
            Ordering::Equal => {
                println!("{}", "PERFECT".green());
                break;
            },
        }
    }
}
