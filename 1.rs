// 1: Write a programme which finds the factorial
// of a number entered by the user.
// (check for all conditions).


use std::io::stdin;


fn factorial(n: i32) -> i32 {
    let mut index: i32 = n;
    let mut result: i32 = 1;

    while index > 0 {
        result = result * index;
        index = index - 1;
    }

    return result;
}


fn main() {
    let mut input_string = String::new();

    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let num: i32 = input_string.trim().parse().expect("Input not an integer");
    let returned_factorial = factorial(num);

    println!("{}", returned_factorial);
}
