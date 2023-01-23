use std::io;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //  Shadowing
    shadowing();
    //  Tuples and Arrays
    tuples_and_arrays();

    // Functions with arguments
    function_with_argument(10, 'm');

    let return_value = function_with_return(10);
    println!("{return_value}");

    if_statements();

    for_loop();

    let num_seq = get_number_input();
    fibonacci_seq(num_seq);
}

fn shadowing() {
    let ix = 10;

    println!("The value of ix is: {}", ix);

    let ix = ix + 5;

    {
        let ix = ix * 10;
        println!("The value of ix in inner scope: {ix}");
    }

    println!("The value of ix: {ix}");

    // 3.2 Data Types

    let y: i8 = "42".parse().expect("Only int8 values allowed!");

    println!("The value of y: {y}");
}

fn tuples_and_arrays() {
    let tup = (5, 10.0, 15);
    let (x, y, z) = tup;
    println!("The tuple tup: {x} , {y}, {z}");

    //  Arrays

    let arr = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Enter index to access[0-{}]:", arr.len() - 1);
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline.");

    let index: usize = index.trim().parse().expect("Only int8 values please!");

    let element: i32 = arr[index];

    println!("The value of arr at index {index}: {element}");
}

fn function_with_argument(ix: i32, unit: char) {
    println!("fn_with_args - The value of ix is : {ix}{unit}");
}

fn function_with_return(ix: i32) -> i32 {
    return ix + 5;
}

fn get_number_input() -> i32 {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line.");

    let input: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => -1,
    };

    input
}

fn if_statements() {
    let num = get_number_input();
    if num < 5 {
        println!("{num} is less than 5!");
    } else if num == 5 {
        println!("{num} is equal to 5.");
    } else {
        println!("{num} is greater than 5");
    }
}

fn for_loop() {
    let mut c: i8 = 0;
    let res = loop {
        c += 1;
        if c == 10 {
            break c * 10;
        }
    };
    println!("Result: {res}");

    for ix in (1..5).rev() {
        println!("{ix}");
    }
}

fn fibonacci_seq(n: i32) {
    let mut a = 0;
    let mut b = 1;
    if n > 1 {
        for _ in 0..n {
            let sum = a + b;
            a = b;
            b = sum;
            println!("{sum}");
        }
    }
}
