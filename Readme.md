## Learning Rust

This repository contains the learning outcome of Rust Programming Language.

## Installation

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

## Check Version

Now check the version using the command:

```sh
$ rustc --version
```

Let's go now you are ready to write code in Rust.

## Write your first Rust Program
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