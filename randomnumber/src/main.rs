extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	println!("echo something :");
	let mut x = String::new();
	let i_Random = rand::thread_rng().gen_range(1,1000000);
	io::stdin().read_line(&mut x).ok().expect("cannot read");
    println!("echoing {}, and random no. {}", x, i_Random);
}
