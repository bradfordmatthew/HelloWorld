use std::io;
mod moon;
mod cowlib;
mod gamelib;
mod second;
mod worldlib;

//cowbull
fn chase_the_herd (guess: &str, number: String, bovines: &mut cowlib::TheHerd) {
    let mut x_index     = 0;
    for x in guess.chars() {
        let mut i_index = 0;
        for i in number.chars() {
            if x == i {
                if x_index == i_index {
                    bovines.herd[x_index].set_bull(x_index, x);
                }
                else{
                    bovines.herd[x_index].set_cow();
                }
            }
            i_index = i_index + 1;
        }
        x_index = x_index + 1;
    }
}

fn main() {
    let mut stdin = io::stdin();
    let number = moon::random_num(1000, 10000);
    println!("-------------------------------------------");
    println!("\t     Welcome to the game cow bull!!!\r\n
    {}\r\n {}\r\n {}\r\n {}\r\n {}\r"
    ,"The object of this game is to guess a number between"
    ,"   1000 and 9999. You will be given the clue of cows or"
    ,"  bulls. A cow is a correct number in the wrong position."
    ,"     A bull is the right number in the right position."
    ,"\r\n\t\t**Type exit to quit the game.**"
    );
    println!("___________________________________________");
    println!("Please enter 4 numerical characters!");

    loop {
        let guess = &mut String::new();
        stdin.read_line(guess);//TODO: How to turn off this warning...
        let guess_trim = guess.trim();//but it is used!

        if guess_trim == "exit"{
            break;
        }

        if guess_trim != "doode!" {
            if guess_trim == number {
                println!("You win!!!!! You guessed: {}", number);
                std::thread::sleep_ms(3000);
                break;
            }

            if guess_trim.len() != 4 {
                println!("Please enter a 4 digit numerical string.");
                continue;
            }
            
            //test and see if this uploads :)
            let mut bovines = cowlib::TheHerd::new();

            chase_the_herd(guess_trim, number.to_string(), &mut bovines);

            println!("Bulls: {}, Cows: {}.", bovines.bull_count(), bovines.cow_count())
        }
        else {
            //individual game loops and a governing loop? Is that a thing?
            if second::game_without_a_name() {
                println!("Woo you played the other game and won!");
                break;
            }
        }
    }
}

//Game play tests
#[test]
fn test_case_1 () {
    let mut bovines = cowlib::TheHerd::new();
    chase_the_herd("0007", "7770".to_string(), &mut bovines);
    assert!(bovines.cow_count() == 4 && bovines.bull_count() == 0);
}

#[test]
fn test_case_2 () {
    let mut bovines = cowlib::TheHerd::new();
    chase_the_herd("9999", "1290".to_string(), &mut bovines);
    assert!(bovines.cow_count() == 3 && bovines.bull_count() == 1);
}
