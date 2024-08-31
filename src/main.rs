use std::io;
use rand::Rng;


fn main() {
    println!("I need a Software job!");

    let farn = convet_celsius(98.0);

    let celsius = convet_farnheit(farn);

    println!("The temperature is {farn} degree celsius");
    println!("The temperature is {celsius} degree farnheit");
    
    println!("The fibonacci for index 5 is {}", generate_fibonacci(5));

    guessing_game();

}

fn convet_celsius (temperature : f32) -> f32{
    return 5.0 * (temperature - 32.0)/9.0
}

fn convet_farnheit (temperature : f32) -> f32{
    return (9.0 * (temperature )/5.0) + 32.0
}

fn guessing_game() {

    println!("Enter a number you want to guess:");

    let num = rand::thread_rng().gen_range(1..=100); 

    loop { 
        let mut guess =  String::new();

        io::stdin().read_line( &mut guess).expect("Expected a number");
        if let Ok(guess) = guess.trim().parse::<i32>(){
            if guess == num{
                println!("Your Guess is correct! {guess}");
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

fn generate_fibonacci(index:i64) -> i64{
    if (index == 0) || (index == 1){
        return 1
    }
    else{
        return generate_fibonacci(index-1) + generate_fibonacci(index-2)
    }
}