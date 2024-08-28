## Learning Rust

This repository contains the learning outcome of Rust Programming Language.

# Table of Contents
1. [Installation](#Installation)
2. [Check Version](#Check-Version)
3. [Write your first Rust Program](#Write-your-first-Rust-Program)
3. [Building Rust app with Cargo](#Building-Rust-app-with-Cargo)


# Installation

For a linux based terminal, enter the following command in your terminal:

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Rust will be now installed in your computer.

Now Run the command:
```sh
source ./~bashrc
```
The changes will be now reflected to your computer.

# Check Version

Now check the version using the command:

```sh
$ rustc --version
```

Let's go now you are ready to write code in Rust.

# Write your first Rust Program
```sh
mkdir rust_program
cd rust_program
touch helloworld.rs
```

Now we are ready to write the code BINGO:

```rs
fn main(){
    println!("Hello, World!");
}
```

To execute the program, write the command in your terminal:
```sh
rustc helloworld.rs
```
Then the executable file will be saved as helloworld in same directory you are working. So you can run:
```sh
./helloworld
```

You can see the output as:
```sh
Hello, World!
```

# Building Rust app with Cargo

Let's create a Rust app with cargo that takes user input and gives output of the number user guesses.

```sh
cargo new guessing_game
```

Then to build the cargo app:
```sh
cd guessing_game
cargo build
```

Now you can run the app using:
```sh
cargo run
```

Lets write the main function for guessing game:

```rs
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess: ");

	let mut guess = String::new();
	
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read the line.");

	println!("You guessed: {} ", guess);
}
```
After we build and run this function again we will get the output:
```sh
    Compiling guessing_game v0.1.0 (/home/rohan/rush-projects/rust/guessing_game)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
        Running `target/debug/guessing_game`
    Guess the number
    Please input your guess: 
```