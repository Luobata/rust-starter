//use ferris_says::say; // from the previous step
//use std::io::{stdout, BufWriter};
use std::f64::consts::PI;
use std::io;

fn main() {
    //let stdout = stdout();
    //let message = String::from("Hello fellow Rustaceans!");
    //let width = message.chars().count();

    //let mut writer = BufWriter::new(stdout.lock());
    //say(message.as_bytes(), width, &mut writer).unwrap();

    println!("start input");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let number = input.trim().parse().expect("input is not a number!");
    let n = 5;
    let slice_PI = format!("{:.n$}", PI, n = number);

    println!("your input is:{}", number);
    println!("PI is:{}", slice_PI)
}
