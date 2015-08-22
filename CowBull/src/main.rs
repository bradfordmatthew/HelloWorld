use std::io;
mod moon;
mod cowlib;

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
    let number = moon::horrid();
    println!("-------------------------------------------");
    println!("\t     Welcome to the game cow bull!!!\r\n
    {}\r
    {}\r
    {}\r
    {}\r
    {}\r"
    ,"The object of this game is to guess a number between"
    ,"1000 and 9999. You will be given the clue of cows or"
    ,"bulls. A cow is a correct number in the wrong position."
    ,"A bull is the right number in the right position."
    ,"\r\n\t\t**exit to quit the game.**"
    );
    println!("___________________________________________");
    println!("Please enter 4 numerical characters!");
    
    loop {
        let mut guess = &mut String::new();
        stdin.read_line(guess);
        let guess = guess.trim();

        if guess == "exit"{
            break;
        }

        if guess == number {
            println!("You win!!!!! You guessed: {}", number);
            std::thread::sleep_ms(3000);
            break;
        }

        if guess.len() != 4 {
            println!("Please enter a 4 digit numerical string. {}", guess.len());
            continue;
        }

        let mut bovines = cowlib::TheHerd::new();

        chase_the_herd(guess, number.to_string(), &mut bovines);

        println!("Bulls: {}, Cows: {}.", bovines.bull_count(), bovines.cow_count())
    }
}

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
