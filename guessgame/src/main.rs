use std::io;

fn main() {
	println!("Guessing game..");
	println!("Please enter a guess : ");
	
	let mut sGuess = String::new();

	io::stdin().read_line(&mut sGuess).ok().expect("failed to read line");
	println!("your guess {}", sGuess);
}
