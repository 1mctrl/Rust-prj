use colored::*;
use std::io;

fn get_nums() -> (i64, i64) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let num1: i64 = input
        .trim()
        .parse()
        .expect("invalid num");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("err");

    let num2: i64 = input
        .trim()
        .parse()
        .expect("not vaild num");

    (num1, num2)
}

fn calculating() {
    let (num1, num2) = get_nums();

    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("write opration");

    let op = op.trim();

    if op == "+" {
        println!(
            "Answer {}",
            (num1 + num2)
                .to_string()
                .green()
        );
    } else if op == "-" {
        println!(
            "Answer {}",
            (num1 - num2)
                .to_string()
                .green()
        );
    } else if op == "*" {
        println!(
            "Answer {}",
            (num1 * num2)
                .to_string()
                .green()
        );
    } else if op == "/" {
        if num2 == 0 {
            println!("{}", "Dividion by 0 is not allowed".red());
        } else {
            println!(
                "Answer {}",
                (num1 / num2)
                    .to_string()
                    .green()
            );
        }
    } else {
        println!("{}", "cant recognize operation, write '+', '-', '*', '/'".red());
    }
}

fn main() {
    loop {
        println!("{}", "----------------\nLETS CALCULATE SOME INTEGERS".blue());
        println!("{}", "Type 'Start' to start ot 'Exit' to exit".blue());

        let mut begin = String::new();
        io::stdin()
            .read_line(&mut begin)
            .expect("err");

        let begin = begin.trim();

        if begin == "Start" {
            calculating();
        } else if begin == "Exit" {
            println!("{}", "goodbye!".magenta());
            std::process::exit(0);
        } else {
            println!("{}", "type exactly 'Start' or 'Exit'!".red());
        }
    }
}
