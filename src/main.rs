use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Guess a letter!");

    let mut selectedWord = ""

    let secretNumber = rand::thread_rng().gen_range(1, 6); //Lowest number is inclusive, higest is exclusive

    match secretNumber {
        1 => selectedWord = "Keyboard",
        2 => selectedWord = "Rust",
        3 => selectedWord = "Space",
        4 => selectedWord = "Java",
        5 => selectedWord = "Quarentine",
    }


    // struct Letter {
    //     character: char, 
    //     revealed: bool,
    // }

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            // let guess: u32 = match guess.trim().parse() {
            //     Ok(num) => num,
            //     Err(_) => continue,
            // };

        println!("You guessed: {}", guess);

        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }

    }
}
