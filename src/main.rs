use std::io;
use rand::Rng;


fn main() {
    println!("I need a Software job!");

    println!("Enter a number you want to guess:");

    let mut guess= String::new();

    let num = rand::thread_rng().gen_range(1..=1).to_string(); 

    loop {
        guess.clear();
        io::stdin().read_line( &mut guess).expect("Expected a number");
        if guess.trim() == num{
            print!("Your guess({}) is correct!", guess.trim());
            break;
        }
        println!("Wrong Guess ({}), try again! ", guess.trim())
    }
}
