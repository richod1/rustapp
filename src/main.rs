use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
// loop added
    loop{ 

    println!("welcome to the guess game");
    println!("input your guess in number");

    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to compile");
    println!("you guess : {}",guess);

    let random_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Random number: {}", random_number);

//    comparing of guess game
let guess:u32=guess.trim().parse().expect("Type a number");

match guess.cmp(&random_number){
    Ordering::Less=>println!("Your guess is Too small"),
    Ordering::Greater=>println!("Your guess is Too Big"),
    Ordering::Equal=>println!("Your guess is Equal"),
}

    }

    
}
