use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    println!("The number of seconds in 3 hours is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Data types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    // Scalar
    // TODO: integer overflow testing
    
    let x = 2.0; // f64 ; 2 -> error
    let y: f32 = 3.0; // f32
    let w = x + y;
    println!("The value of w is: {w}");

    // Numeric Ops
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.0 / 32.0;
    let truncated = -5 / 3; 
    let remainder = 43 % 5;
    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");

    // Booleans
    let t = true;
    let f: bool = false;
    let truth = t || f;
    println!("The truth is: {truth}");

    // Chars
    let character = 'z';
    let crt: char = 'ℤ';
    let heart_eyed_cat = '�';

    println!("{character}->{crt}->{heart_eyed_cat}");

    // Compound Types
    //  tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    let first_value = tup.0;
    println!("The first value in tuple is: {first_value}");

    //  arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_same_val = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
    let sum = arr[1] + arr_same_val[1];
    println!("Sum is: {sum}");

    // invalid array access
    let a = [1, 2 ,3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
