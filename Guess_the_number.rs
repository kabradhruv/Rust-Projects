use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
    println!("Guess the number");
    let secret_num : u32 = rand::thread_rng().gen_range(1,101);
    println!("{}","Try to guess the right number == \n".blue());

    loop{
        println!("Please input your guess : ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to readline");

        let guess : u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guessed : {}",guess);

        match guess.cmp(&secret_num){
            Ordering::Less => println!("{}","It's small.Try Again ! ".red()),
            Ordering::Greater => println!("{}","It's bigger.Try Again ! ".red()),
            Ordering::Equal => {
                println!("{}","Congratulation! You guessed the right number".green());
                break;    
            },
        }
    }
}
