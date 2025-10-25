use colored::*;
use std::io;

fn get_num() -> (i128, i128) {
    println!("{}", "Type 1st number".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err");
    let num1: i128 = input.trim().parse().expect("invalid num");

    println!("{}", "Type 2nd number".yellow());
    input.clear();
    io::stdin().read_line(&mut input).expect("err");
    let num2: i128 = input.trim().parse().expect("err");

    (num1, num2)
}

fn calc() {
    let (num1, num2) = get_num();

    let mut operand = String::new();
    io::stdin().read_line(&mut operand).expect("err");

    let operand = operand.trim();

    if operand == "+" {
        println!("Summ of integers = {}", (num1 + num2).to_string().green());
    } else if operand == "-" {
        println!(
            "Substraction of integers = {}",
            (num1 - num2).to_string().green()
        );
    } else if operand == "*" {
        println!(
            "Multiplication of inetgers = {}",
            (num1 * num2).to_string().green()
        );
    } else if operand == "/" {
        if num2 != 0 {
            println!(
                "Division of integers = {}",
                (num1 / num2).to_string().green()
            );
            if num1 % num2 != 0 {
                println!("reminder = {}", (num1 % num2).to_string().green());
            }
        } else {
            println!("{}", "Division by 0 is no allowed".red());
        }
    } else {
        println!(
            "{}",
            "Can't recognize operand, type '+'; '-'; '*'; '/'".red()
        );
    }
}

fn euclid() {
    let (mut num1, mut num2) = get_num();

    while num2 != 0 {
        let r = num1 % num2;
        println!(
            "{}",
            format!("Reminder of {num1} and {num2} = {r}").magenta()
        );

        num1 = num2;
        num2 = r;
    }

    println!("{}", format!("GCD of these numbers = {num1}").green());
}

fn main() {
    loop {
        println!("{}", "Lets do some math\n__________________".blue());

        println!("1. Type 'EUCLID' to find GCD of integers;");
        println!("2. Type 'CALCULUS' to do basic calculations with integers only;");
        println!("3. Type 'QUIT' to quit");

        let mut inpt = String::new();
        io::stdin().read_line(&mut inpt).expect("err");

        let inpt = inpt.trim();

        if inpt == "EUCLID" {
            println!(
                "{}",
                "1. Type 1st int then RET.\n2. Type 2nd int then RET.".magenta()
            );
            euclid();
        } else if inpt == "CALCULUS" {
            println!(
                "{}",
                "1. Type 1st int then RET.\n2. Type 2nd int then RET.\n3. Type operand then RET"
                    .magenta()
            );
            calc();
        } else if inpt == "QUIT" {
            println!("{}", "hope to see ya again!".red());
            std::process::exit(0);
        } else {
            println!("{}", "you've write something wrong, write as I said".red());
        }
    }
}
