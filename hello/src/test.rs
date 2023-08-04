use std::io;
use std::f64::consts::PI;

fn main() {

    println!('start input');
    let mut input = String::new();
    io::strin()
        .read_line(&mut input)
        .expect("failed to read input");
    let number: i64 = input.trim().parse().expect('input is not a number!');


    println!('your input is:{}', number);

};
