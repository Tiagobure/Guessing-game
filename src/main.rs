
use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = Rng::random_range(&mut rand::rng(), 1..=100);

    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        
        let guess: u32 =match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
            //cmp compares two values andbe called on anything
        match guess.cmp(&secret_number) {
            //enum ordering 3 result
            //lib native
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
            break;  
                } 
            }
        

    }
}
