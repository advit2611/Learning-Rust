use std::{fmt::Debug, io};
use rand::Rng;


fn main() {
    println!("I need a Software job!");
    
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value of coin is {}", values_to_coin(coin));
    
    let rect = Rectangle{
        width: 32,
        height: 54,
    };

    let square = Rectangle::square(rect.width);

    let area_of_rectangle = calculate_area(&rect);
    let area_of_square = square.calculate_area();
    println!("The area for width {} and height {} is {} ", rect.width, rect.height, area_of_rectangle);
    println!("The area for length {} is {} ", square.width, area_of_square);
    println!("Can Rectangle hold the square, {}", square.can_hold(&rect));
    println!("{:#?}",rect);

    using_struct();

    let mut s1 = String::from("hello world!"); // This stores string in heap while `let s1 = "Hi"` stores in stack


    let len = add_h_and_calculate_length(&mut s1); // Function mutates and pushes an 'h' to the string

    println!("length of {} is {}", &s1, len);

    println!("First word is {}", find_first_word(&s1));

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

fn add_h_and_calculate_length(s: &mut String) -> usize{
    s.push_str("h");
    let result = s.len();
    result
}

fn find_first_word(s: &str) -> &str{
    let input = s.as_bytes();
    for (i, &letter) in input.iter().enumerate(){
        if letter == b' '{
            return &s[0..i]
        }
    }
    return &s[..];

}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    number_of_sign_ins: u64
}

fn using_struct(){
    let user_1 = User {
        username: String::from("Advit"),
        email: String::from("abc@gmail.com"),
        active: false,
        number_of_sign_ins: 1,
    };

    println!("The username is {} with {} has an active account {} with number of sign ins {}", user_1.username, user_1.email, user_1.number_of_sign_ins, user_1. active);

    let user_2 = User {
        email: String::from("xyz@gmail.com"),
        ..user_1
    };

    println!("user 2 is {:#?}", user_2);

    struct Color (u8, u8, u8);
    fn set_bg_color(color: Color){
        println!("The background is set to color with R={}, G={}, B={}", color.0, color.1, color.2);
    }
    let red = Color(100, 0, 0);
    set_bg_color(red);
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn calculate_area(rect: &Rectangle) -> u32 {
    let area = rect.width * rect.height;
    area
}

impl Rectangle {

    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn square(side: u32) -> Self{
        Self {
            width: side,
            height: side,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.height >= other.height && self.width >= other.width
    }
    
}
#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
}

enum Coin{
    Penny,
    Nicklel,
    Dime,
    Quarter(UsState),
}

fn values_to_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("We have a penny");
            1
        },
        Coin::Nicklel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) =>{
            println!("Got an Alaskan Quarter");
            25
        },
        Coin::Quarter(state) => {
            println!("This coin is from {:#?}", state);
            25
        }
    }
}