use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn game() -> io::Result<()> {
    println!("Guess a number between 1..=100 inclusive");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your number");

        let mut guessed_number = String::new();
        io::stdin().read_line(&mut guessed_number)?;

        let Ok(guessed_number) = guessed_number.trim_end().parse::<u8>() else {
            println!("Input a number between 1..=100 inclusive");
            continue;
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
    Ok(())
}

/// # Errors
///
/// Will return `Err` if there is an error reading a line from stdin
pub fn start() -> io::Result<()> {
    game()?;
    Ok(())
}
