use std::io;
mod moon;
mod cowlib;

fn main() {
    let mut stdin = io::stdin();
    let number = moon::horrid();

    let mut guess = &mut String::new();

    loop {
        guess.clear();

        println!("Horrid: {}", number);

        stdin.read_line(guess);
        let guess = guess.trim();

        if guess == "exit"{

            break;
        }

        if guess == number {
            println!("You win!!!!!");
            break;
        }

        if guess.len() != 4 {
            println!("Please enter a number between 1000 and 9999. {}", guess.len());
            continue;
        }

        let mut x_index     = 0;
        let mut bull_count  = 0;
        let mut cow_count   = 0;

        for x in guess.chars() {
            let mut i_index = 0;
            for i in number.chars() {

                println!("x: {}, i: {}. x_index: {}, i_index: {}", x, i, x_index, i_index);

                if x == i {
                    if x_index == i_index {
                        bull_count = bull_count + 1;
                    }
                }

                i_index = i_index +1;
            }
            x_index = x_index +1;
        }

        println!("Bulls: {}, Cows: {}.", bull_count, cow_count)
    }
}
