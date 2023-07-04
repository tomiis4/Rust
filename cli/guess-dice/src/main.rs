use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        println!("Rolling dice..");
        println!("Guess what number I got.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .unwrap();

        // removes \n
        guess.pop();

        if roll().to_string() == guess {
            println!("You are right!");
        } else {
            println!("You are not right!");
        }

        print!("\n")
    }
}

fn roll() -> u8 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u8;

    time % 6 + 1
}
