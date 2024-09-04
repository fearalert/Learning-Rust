use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter number a:");
    io::stdin().
        read_line(&mut a).
        expect("Failed to read the line");
    
    let number_a: u32 = a.trim().parse().expect("Please enter a valid u32 number");

    println!("Enter number b:");
    io::stdin().
        read_line(&mut b).
        expect("Failed to read the line");

    let number_b: u32 = b.trim().parse().expect("Please enter a valid u32 number");

    let sum = number_a + number_b;

    println!("The sum of the given number {a} and {b} is:");
    println!("{sum}");
}
