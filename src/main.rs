use std::io;
use std::cmp::Ordering;
use rand::Rng;
use console::Style;

fn main() {
    print!("{}[2J", 27 as char);
    guessing_game();
}

fn guessing_game() {
    
    let red = Style::new().red();
    let cyan = Style::new().cyan();
    let green = Style::new().green();
    let yellow = Style::new().yellow();
    
    println!("{}", yellow.apply_to("Guess the Number 1 through 10!"));

    let rng_num = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("{}", cyan.apply_to("Please input your guess"));

        let mut guessed_num = String::new();

        io::stdin()
            .read_line(&mut guessed_num)
            .expect("Failed to read your input!");

        // Capture user input and parse as an u32
        let guessed_num: u32 = match guessed_num.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guessed_num.cmp(&rng_num) {
            Ordering::Less      => println!("{}", red.apply_to("Your guess was too low!")),
            Ordering::Greater   => println!("{}", red.apply_to("Your guess was too high!")),
            Ordering::Equal     => {
                println!("{}", green.apply_to("You guessed it!"));
                break;
            }
        }
    }
}
