use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();  // mut -> mutable String :: new -> new는 String의 연관함수임을 나타낸다
    io::stdin().read_line(&mut guess)
       .expect("Failed to readLine");

}

