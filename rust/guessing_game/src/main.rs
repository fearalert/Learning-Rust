use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");


	// Generating a random secret_number to be guessed. For this we use rand
	let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

	let mut guess = String::new();
	
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read the line.");

	let guess: u32 = guess.trim().parse().expect("Please type a number");

	println!("You guessed: {} ", guess);

	match guess.cmp(&secret_number){
		Ordering::Less => println!("Too Small");
		Ordering::Greater => println!("Too Large");
		Ordering::Equal => println!("You Win!")
	}
}
