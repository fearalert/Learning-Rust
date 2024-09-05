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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(perform_operation(1.0,-2.0,1), -1.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(perform_operation(1.0,-2.0,2), 3.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(perform_operation(1.0,-2.0,3), -2.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(perform_operation(1.0,-2.0, 4), -0.5);
    }

    #[test]
    fn test_invalid_operator() {
        assert!(f64::is_nan(perform_operation(10.0, 2.0, 5)));
    }

    fn perform_operation(a: f64, b: f64, op: i32) -> f64 {
        match op {
            1 => a + b,
            2 => a - b,
            3 => a * b,
            4 => a / b,
            _ => f64::NAN,
        }
    }
}
