use std::io;

fn main() {
    let mut inpt = String::new();
        io::stdin()
        .read_line(&mut inpt)
        .expect("err");
    let num: u64 = inpt.trim().parse().expect("err");

    let muner: Vec<u64> = (1..=1000).collect();

    for i in muner.iter() {
        if num % i == 0 {
            println!("{}", i);
        }
    }
}
