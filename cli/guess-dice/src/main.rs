use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        println!("Rolling dice..");
        println!("Guess what number I got.");

        let mut guess_line = String::new();

        std::io::stdin()
            .read_line(&mut guess_line)
            .unwrap();

        // removes \n
        guess_line = guess_line.trim().to_string();

        let guess = match guess_line.parse::<u8>() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter correct value\n");
                continue;
            }
        };

        if roll() == guess {
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
