use std::io;

fn main() {
    println!("Hello, welcome to guess game in rust");
    println!("pls input your guess");

    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");

    println!("you guess : {}",guess);
}
