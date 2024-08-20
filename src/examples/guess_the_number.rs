use std::io;
use rand::Rng;

pub struct GuessTheNumber {}

impl GuessTheNumber {
    
    pub fn guess(&self) {
        
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_guess() {
        let guess_the_number = GuessTheNumber{};
        guess_the_number.guess();
    }
}
