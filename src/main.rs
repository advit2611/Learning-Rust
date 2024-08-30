use std::io;
use rand::Rng;


fn main() {
    println!("I need a Software job!");

    println!("Enter a number you want to guess:");

    let num = rand::thread_rng().gen_range(1..=100); 

    loop { 
        let mut guess =  String::new();

        io::stdin().read_line( &mut guess).expect("Expected a number");
        if let Ok(guess) = guess.trim().parse::<i32>(){
            if guess == num{
                print!("Your Guess is correct! {guess}");
                break
            }
            else if guess > num{
                println!("Guess a little lower!")
            }
            else{
                println!("Guess a little higher!")
            }
        }
        else {
            println!("Enter a numerical integer...")
        }
        println!("Wrong Guess ({}), try again! ", guess.trim())
    }
}
