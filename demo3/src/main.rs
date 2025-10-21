use std::io;

fn main() {
    println!("summation");

    let mut num1 = String::new();
    println!("input 1st number:");

    io::stdin()
        .read_line(&mut num1)
        .expect("i dunno");

    let mut num2 = String::new();
    println!("input 2nd number");

    io::stdin()
        .read_line(&mut num2)
        .expect("i duno");

    let num1: i32 = num1
        .trim()
        .parse()
        .expect("its not a number");

    let num2: i32 = num2
        .trim()
        .parse()
        .expect("its not a number");

    let sum = num1 + num2;
    let sub = num1 - num2;
    let div = num1 / num2;
    let time = num1 * num2;
    println!("sum: {}, sub: {}, time: {}, div: {}", sum, sub, time, div);
}
