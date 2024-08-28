# Learning Rust

This repository contains the learning outcome of Rust Programming Language.

# Table of Contents
1. [Installation](#Installation)
2. [Check Version](#Check-Version)
3. [Write your first Rust Program](#Write-your-first-Rust-Program)
4. [Building Rust app with Cargo](#Building-Rust-app-with-Cargo)
5. [Rust Variables And Mutability](#Rust-Variables-And-Mutability)


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

# Rust Variables And Mutability

By default, variables in Rust are immutable. Let us create a cargo named variables to dive into rust variables.
```sh
cargo new variables
```
Now change the directory to variables:
```sh
cd variables
```
Now inside of ```src/main.rs```, if you write the code as:
```rs
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
You will encounter the following error while performing ```cargo run```:
```sh
Compiling variables v0.1.0 (/home/rohan/rush-projects/rust/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

To fix this error, add the keyword ```mut``` after the keyword ```let``` when declaring the variable. For
example:

```rs
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Shadowing
Alternatively, you might consider initializing a new variable: either with a new bound name or (by ```shadowing```) with the bound name of your
existing variable. For example:

```rs
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
}
```

This will give the output as:
```sh
Compiling variables v0.1.0 (/home/rohan/rush-projects/rust/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

## Constant Variables
Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

You are not allowed to use ```mut``` with ```const```.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Constant are valid for the entire time the program runs.

Example:
```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```