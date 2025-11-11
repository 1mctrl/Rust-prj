
use std::io;

struct Numer {
    height: f64,
    width: f64,
}
fn main () {
    getnums();
}

fn getnums()  {
    let mut inpt = String::new();

    io::stdin()
        .read_line(&mut inpt)
        .expect("err");
    let num1 = inpt.trim().parse().expect("err");

    inpt.clear();
    io::stdin()
        .read_line(&mut inpt)
        .expect("err");
    let num2 = inpt.trim().parse().expect("err");

    let var = Numer {
        height: num1,
        width: num2,
    };
    calcul(&var)
}

fn calcul (cont: &Numer) {
    println!("area: {}", cont.height * cont.width);
}
