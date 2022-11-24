use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {

    println!("Guess the number !");
    let mut secret_number = rand::thread_rng().gen_range(1..101);


    loop {
        
        let mut play = String::new();
        
        //print!("Please enter your guess : ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
        
        let guess: u32 = match guess.trim().parse(){ 
            Ok(num) => num,
            Err(_)=> continue,
        };
                            
        println!("You guess nomber: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal =>  { 
                println!("Well done !");
                println!("Want to play again ? (1 Yes/ 0 Nope) ");
                io::stdin()
                .read_line(&mut play)
                .expect("Failed to read");
        
                let play: u32 = match play.trim().parse(){ 
                Ok(num) => num,
                Err(_)=> continue
                };
                if play != 1 {
                    break;        
                }
                println!("Guess the number !");
                secret_number = rand::thread_rng().gen_range(1..101);
            }
        }
    

    }
}
