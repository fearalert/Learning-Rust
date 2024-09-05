use std::io;

fn main() {
    let result:f64;

    println!("Enter first number:");

    let a: f64 = input_parser();

    if f64::is_nan(a){
        return;
    }

    println!("Enter second number:");
    let b: f64 = input_parser();

    if f64::is_nan(b){
        return;
    }

    println!("Please enter the operator");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("Select the number associated with the operations above.");

    let op: f64 = input_parser();

    if f64::is_nan(op){
        return;
    }

    let op:i32 = op as i32;

    match op {
        1 => result = a+b,
        2 => result = a-b,
        3 => result = a*b,
        4 => result = a/b,
        _ => {
            println!("Invalid operator selection");
            return;
        }
}
    println!("The result is {}", result);
}

fn input_parser() -> f64{
    let mut x = String::new();
    io::stdin().
        read_line(&mut x).
        expect("Failed to read the line");
    
    let x: f64 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return f64::NAN;
        }
    };

    return x;
}