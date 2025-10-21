use std::io;

fn compare() {
    println!("type number");

    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("numbers!");

    let num = num.trim();

    let numint: i32 = match num.parse() {
        Ok(good) => good,
        Err(bad) => {
            println!("error {}", bad);
            return;
        }
    };

    println!("type another");

    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("numbers!");

    let num2 = num2.trim();

    let num2int: i32 = match num2.parse() {
        Ok(good) => good,
        Err(bad) => {
            println!("error {}", bad);
            return;
        }
    };

    if numint > num2int {
        println!("---First number bigger than second--- {} ", num);
    } else if num2int > numint {
        println!("---Second number is bigger than first--- {}", num2);
    } else {
        println!("===Numbers are equal=== {} {}", numint, num2int);
    }
}

fn main() {
    loop {
        println!(
            "-----------------------\nWELCOME TO NUMBERS COMPARATOR!\n+++Type 'Start' to start testing+++\n---Type 'Exit' to exit---"
        );

        let mut start = String::new();
        io::stdin()
            .read_line(&mut start)
            .expect("err");

        let start = start.trim();

        if start.eq_ignore_ascii_case("Exit") {
            println!("bye");
            std::process::exit(0);
        } else if start == "Start" {
            compare();
        } else {
            println!("type exactly 'Start' or 'Exit'!");
        }
    }
}
