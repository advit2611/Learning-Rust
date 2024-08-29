use std::io;

fn main() {
    println!("I need a Software job!");

    println!("Enter a number you want to guess:");

    let mut guess= String::new();

    let num = 82; 

    loop {
        io::stdin().read_line( &mut guess).expect("Expected a number");
        if guess == num.to_string(){
            print!("Your guess ({guess}) is correct!");
            break;
        }
        println!("Wrong Guess, try again!")
    }
}
