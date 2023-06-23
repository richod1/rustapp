use std::io;
use rand::Rng;

fn main() {

    let random_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Random number: {}", random_number);
    // println!("Hello, welcome to guess game in rust");
    // println!("pls input your guess");

    // let secret_number=rand::thread_rng().gen_range(1,101);

    // println!("secreate gen number is :{}",secret_number);

    // let mut guess=String::new();
    // io::stdin().read_line(&mut guess).expect("failed to read");

    // println!("you guess : {}",guess);

    // // generating random numbers for 1 to 100

    
}
