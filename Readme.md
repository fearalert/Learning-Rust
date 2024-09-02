# Learning Rust

This repository contains the learning outcome of Rust Programming Language.

# Table of Contents
1. [Installation](#Installation)
2. [Check Version](#Check-Version)
3. [Write your first Rust Program](#Write-your-first-Rust-Program)
4. [Building Rust app with Cargo](#Building-Rust-app-with-Cargo)
5. [Rust Variables And Mutability](#Rust-Variables-And-Mutability)
6. [Data Types](#Data-Types)


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

## Starting the function
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

In the above code, ```stdin``` function from the ```io``` module, which allow us to handle user input.
```rs
 io::stdin()
        .read_line(&mut guess)
```
The ```stdin``` function returns an instance of ```std::io::Stdin```, which is a type that represents a handle to the standard input for your terminal.

And, we handle the potential error with:
```rs
.expect("Failed to read line");
```

Printing the values with:
```rs
println!("You guessed: {}", guess);
```

### Testing the first part till now
After we build and run this function again we will get the output:
```sh
    Compiling guessing_game v0.1.0 (/home/rohan/rush-projects/rust/guessing_game)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.91s
        Running `target/debug/guessing_game`
    Guess the number
    Please input your guess: 
```

## Generating a Random Secret Number
Now, let us add a random secret number that we will be guessing in this game:
```rs
let secret_number = rand::thread_rng().gen_range(1..=100);
```
For this we will need to use a crate to get more functionality. Here ```rand``` library crate, which contains code that is intended to be used in other programs and can’t be executed on its own. The project we are building is a binary crate, which is an executable. Remember, ```crate``` is a collection of Rust Source Code Files.

The code now looks as:
```rs
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

	let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

	let mut guess = String::new();
	
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read the line.");

	println!("You guessed: {} ", guess);
}
```
For this code to build and run modify the ```Cargo.toml``` file in your ```guessing_game``` directory.
From This:
```rs
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
```

To This:
```rs
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

## Comparing the secret number to guess
Now that we have user input and a random number, we can compare them.
First we add another ```use``` statement, bringing a type called ```std::cmp::Ordering``` into scope from the standard library. The Ordering type is another ```enum``` and has the variants ```Less```, ```Greater```, and ```Equal```. These are the three outcomes that are possible when you compare two values.
```rs
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

	println!("You guessed: {} ", guess);

	match guess.cmp(&secret_number){
		Ordering::Less => println!("Too Small");
		Ordering::Greater => println!("Too Large");
		Ordering::Equal => println!("You Win!")
	}
}
```
While running this code with ```cargo build```, the output is:
```sh
cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///rust/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: method defined here
  --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/cmp.rs:836:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
```

When we wrote ```let mut guess = String::new()```, Rust was able to infer that guess should be a ```String``` and didn’t make us write the type. The ```secret_number```, on the other hand, is a ```number``` type. This caused the mismatched type error. To solve this add the below line to your code:

```rs
let guess: u32 = guess.trim().parse().expect("Please type a number");
```

We have created a variable named ```guess```. But the program already have a variable named ```guess```. But helpfully Rust allows us to shadow the previous value of ```guess``` with a new one. ```Shadowing``` lets us reuse the ```guess``` variable name rather than forcing us to create two unique variables.

The `main.rs` is now:
```rs
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
```
Now while performing ```cargo run```, we get the output:
```sh
Compiling guessing_game v0.1.0 (file:///rust/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 45
Please input your guess.
  70
You guessed: 70
Too big!
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

# Data Types
Rust is a statically typed language (This means, at the compile time it must know the data types of every variable). Every Value in Rust is of a certain data type.
Two data subtypes are: ```scalar``` and ```compound```.

## Scalar Types
A scalar type represents a single value. Rust has ```four``` primary scalar types:


```1. Integer```
An ```integer``` is a number without a fractional component.
```unsigned integer``` start with ```u``` (```signed integer``` types start with ```i``` instead of ```u```) that takes up ```32 bits``` of space.
```Signed``` and ```unsigned``` refer to whether it’s possible for the number to be negative. In other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).
```sh
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
```
Each ```signed``` variant can store numbers from ```-(2n - 1)``` to ```2n - 1 - 1``` inclusive, where ```n``` is the number of bits that variant uses. So an ```i8``` can store numbers from ```-(27)``` to ```27 - 1```, which equals ```-128 to 127```. ```Unsigned``` variants can store numbers from ```0``` to ```2n - 1```, so a ```u8``` can store numbers from ```0 to 28 - 1```, which equals ```0 to 255```.

### Integer Overflow in Rust

In Rust, integer overflow occurs when an arithmetic operation results in a value that exceeds the range of the integer type. Rust handles integer overflow with different approaches depending on whether the code is running in debug or release mode.

### Debug Mode

In debug mode, Rust performs checks to prevent integer overflow. If an overflow is detected, the program will panic, which means it will terminate with an error message. This behavior is intended to help developers catch bugs during development.

Example:
```rs
fn main() {
    let x: u8 = 255;
    let y = x + 1; // This will cause a panic in debug mode
    println!("Result: {}", y);
}
```

### Handling overflow Explicitly
```wrapping_add```, ```wrapping_sub```, ```wrapping_mul```, etc.: These methods perform arithmetic operations with wrapping behavior.
```rs
fn main() {
    let x: u8 = 255;
    let y = x.wrapping_add(1); // Explicit wrapping behavior
    println!("Result: {}", y); // Output will be 0
}
```



```checked_add```, ```checked_sub```, ```checked_mul```, etc.: These methods return an Option that is None if overflow occurs.

```rs
fn main() {
    let x: u8 = 255;
    let y = x.checked_add(1); // Returns `None` on overflow
    match y {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred"),
    }
}
```


```saturating_add```, ```saturating_sub```, ```saturating_mul```, etc.: These methods return the maximum or minimum value of the type if overflow occurs.
```rs
fn main() {
    let x: u8 = 255;
    let y = x.checked_add(1); // Returns `None` on overflow
    match y {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred"),
    }
}
```