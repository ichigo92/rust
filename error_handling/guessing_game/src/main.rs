#[allow(dead_code)]
use std::io;
use std::cmp::Ordering;
// crate::trait
use rand::Rng;


#[derive(Debug)]
pub struct Guess {
	value: i32
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
	    	println!("Guess value must be between 1 and 100, got {}.", value);
	    }

	    Guess { value }
	}

	pub fn value(&self) -> i32 {
		self.value
	}
}


fn main() {

	println!("Guess the number!");

    // generate a random number
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {:?}", secret_number);

    loop {
    	println!("Please input your guess.");

	    let mut guess = String::new();
	    io::stdin()
	        .read_line(&mut guess)
	        .expect("Failed to read line");

	    let guess: i32 = match guess.trim().parse() {
	        Ok(num) => num,
	        Err(_) => continue,
	    };

	    // use Guess instead of checking it here
	    /*if guess < 1 || guess > 100 {
	    	println!("The secret number will be between 1 and 100.");
	    	continue;
	    }*/

	    let num: Guess = Guess::new(guess);
	    let guess = num.value();

	    // println!("You guessed: {}", guess);

	    match guess.cmp(&secret_number) {
	        Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => {
	            println!("You win!");
	            break;
	        }
	    }
    }
}
