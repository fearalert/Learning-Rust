use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter number a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read the line");

    let number_a: u32 = a.trim().parse().expect("Please enter a valid u32 number");

    println!("Enter number b:");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read the line");

    let number_b: u32 = b.trim().parse().expect("Please enter a valid u32 number");

    match number_a.checked_sub(number_b) {
        Some(difference) => {
            println!("The difference of the given numbers {} and {} is: {}", number_a, number_b, difference);
        }
        None => {
            println!("Overflow occurred: b is greater than a, so the subtraction cannot be performed.");
        }
    }
}
