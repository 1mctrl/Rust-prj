//i hope you will input as big integers as i128 can hold
use colored::*;
use std::io;
fn get_num() -> (i128, i128) {
    //take first var m
    println!("{}", "Type first number".yellow());
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("err");
    let m: i128 = num1.trim().parse().expect("cant make int");

    //take second var n
    println!("{}", "Type second number".yellow());
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("err");
    let n: i128 = num2.trim().parse().expect("cant make int");

    (m, n)
}

fn calc_algo() {
    let (mut m, mut n) = get_num();

    while n != 0 {
        let r = m % n;
        println!(
            "{}",
            format!("--Remainder of {m} and {n} = {r}\n___________________________").magenta()
        );
        m = n;
        n = r;
    }
    println!("{}", format!("GCD of these numbers = {m}").green());
}

fn main() {
    loop {
        println!(
            "{}",
            "Lets find Greatest Common Divisor of 2 integers".blue()
        );
        println!("{}", "_____________________________".blue());

        println!("{}", "Type 'BEGIN' to start or 'LEMMEGO' to exit".cyan());

        let mut begin = String::new();
        io::stdin().read_line(&mut begin).expect("err");

        let begin = begin.trim();

        if begin == "BEGIN" {
            calc_algo();
        } else if begin == "LEMMEGO" {
            println!("{}", "Ok, Bye".red());
            std::process::exit(0);
        } else {
            println!("{}", "you should write as I said to start :0".yellow());
        }
    }
}
