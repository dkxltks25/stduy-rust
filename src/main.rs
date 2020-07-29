extern crate rand;

// rand -> 외부에 의존하는 크레이트
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// use line 내에 선언되어야만 모든 스코프 내에서 사용 가능하다.

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("check: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let _guess: u32 =match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}", _guess);
        match _guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}