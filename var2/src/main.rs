use std::io;

fn main() {
    println!("im testing tup");

    let tup: (i32, f32, u32) = (-21, 2.34, 2);
    let (sign, flo, unsign) = tup;

    let mut inpt = String::new();

    io::stdin().read_line(&mut inpt).expect("err");

    println!("types here: {} {} {}", sign, flo, unsign);
}
