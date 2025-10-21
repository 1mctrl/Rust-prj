use std::io;

fn main() {
    println!("type your ipnput");

    let mut dem = String::new();
    io::stdin()
        .read_line(&mut dem)
        .expect("no-no you're wrong");

    let dem = dem.trim();

    let dem2: i32 = match dem.parse() {
        Ok(good) => good,
        Err(bad) => {
            println!("no-no you wrong again {}", bad);
            return;
        }
    };

    println!("your num is {}", dem2)
}
