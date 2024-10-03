use hello_cargo::authenticate;
use hello_cargo::Credentials;
use rand::Rng;
use std::collections::HashMap;
use std::{fmt::Debug, io};

fn main() {
    println!("I need a Software job!");

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x:T, y:T) -> Point<T>{
            Point { x, y }
        }
    }

    impl Point<i32> {
        fn distance_from_origin(&self) -> f32 {
            (((self.x).pow(2) + (self.y).pow(2)) as f32).powf(0.5)
        }
    }

    let p = Point::new(1, 1).distance_from_origin();
    println!("Distance: {}", p);

    fn get_largest<T: std::cmp::PartialOrd>(arr: &Vec<T>) -> &T {
        let mut res = &arr[0];
        for item in arr {
            if item >= res {
                res = item
            }
        }
        return res;
    }
    
    let arr = vec![1, 2, 3];
    println!("Largest of {:?}, is {:?} ", arr, get_largest(&arr));
    fn divide(x: i32, y: i32) -> Result<i32, String> {
        if y == 0 {
            return Err(String::from("Divided by 0"));
        }
        Ok(x / y)
    }

    let divided_result = match divide(3, 0) {
        Ok(num) => num,
        Err(error) => {
            println!("{error}");
            -1
        }
    };

    println!("The quotient is {}", divided_result);

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 25);
    scores.insert(String::from("yellow"), 10);

    // One way of updating a value with .unwrap_or()
    scores.insert(
        String::from("blue"),
        scores.get(&String::from("blue")).copied().unwrap_or(0) + 5,
    );
    // Second way of updating a value in map with match case with dereferencing to get the actual value
    match scores.get(&String::from("yellow")) {
        Some(score) => {
            scores.insert(String::from("yellow"), *score + 10);
        }
        None => {
            scores.insert(String::from("yellow"), 5);
        }
    }
    println!("hasmap is {:#?}", scores);

    let text = "hello I am an applicant for an organization!";
    let mut word_count: HashMap<char, i32> = HashMap::new();

    for word in text.chars() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count is {:#?}", word_count);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    let s3 = format!("{s1}{s2}"); // Does not take ownership of either variable
    let s4 = s1 + &s2; // Takes ownership of s1

    let s5: String = String::from("नमस्ते").chars().take(2).collect();

    println!("Concatenation of s1 and s2 is {s3}");
    println!("Concatenation of s1 and s2 is {s4}");
    println!("First two characters of s5 non-ASCII string is {}", s5);

    let vec: Vec<i32> = vec![5, 4, 6, 2];

    let fourth_value = match vec.get(30) {
        Some(value) => value,
        None => {
            println!("Index out of Bounds");
            &-1
        }
    };
    println!("Fourth value for {:?} is {}", vec, fourth_value);

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let cells = vec![
        SpreadSheetCell::Int(32),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("hello")),
    ];
    match cells.get(1) {
        Some(SpreadSheetCell::Int(value)) => {
            println!("The integer value is {:?}", value)
        }
        Some(_) => {
            println!("Not an Integer value")
        }
        None => {
            println!("Index out of bounds")
        }
    }

    let cred = Credentials {
        username: String::from("Advit"),
        password: String::from("password"),
    };

    println!("{:#?}", authenticate(cred));

    let coin1 = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Quarter(UsState::Alaska);
    let coin3 = Coin::Penny;
    let coin4 = Coin::Dime;
    let coin5 = Coin::Nicklel;
    println!("Value of coin is {}", values_to_coin(coin1));
    println!("Value of coin is {}", values_to_coin(coin2));
    println!("Value of coin is {}", values_to_coin(coin3));
    println!("Value of coin is {}", values_to_coin(coin4));
    println!("Value of coin is {}", values_to_coin(coin5));

    let rect = Rectangle {
        width: 32,
        height: 54,
    };

    let square = Rectangle::square(rect.width);

    let area_of_rectangle = calculate_area(&rect);
    let area_of_square = square.calculate_area();
    println!(
        "The area for width {} and height {} is {} ",
        rect.width, rect.height, area_of_rectangle
    );
    println!(
        "The area for length {} is {} ",
        square.width, area_of_square
    );
    println!("Can Rectangle hold the square, {}", square.can_hold(&rect));
    println!("{:#?}", rect);

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

fn convet_celsius(temperature: f32) -> f32 {
    return 5.0 * (temperature - 32.0) / 9.0;
}

fn convet_farnheit(temperature: f32) -> f32 {
    return (9.0 * (temperature) / 5.0) + 32.0;
}

fn guessing_game() {
    println!("Enter a number you want to guess:");

    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Expected a number");
        if let Ok(guess) = guess.trim().parse::<i32>() {
            if guess == num {
                println!("Your Guess is correct! {guess}");
                break;
            } else if guess > num {
                println!("Guess a little lower!")
            } else {
                println!("Guess a little higher!")
            }
        } else {
            println!("Enter a numerical integer...")
        }
        println!("Wrong Guess ({}), try again! ", guess.trim())
    }
}

fn generate_fibonacci(index: i64) -> i64 {
    if (index == 0) || (index == 1) {
        return 1;
    } else {
        return generate_fibonacci(index - 1) + generate_fibonacci(index - 2);
    }
}

fn add_h_and_calculate_length(s: &mut String) -> usize {
    s.push_str("h");
    let result = s.len();
    result
}

fn find_first_word(s: &str) -> &str {
    let input = s.as_bytes();
    for (i, &letter) in input.iter().enumerate() {
        if letter == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    number_of_sign_ins: u64,
}

fn using_struct() {
    let user_1 = User {
        username: String::from("Advit"),
        email: String::from("abc@gmail.com"),
        active: false,
        number_of_sign_ins: 1,
    };

    println!(
        "The username is {} with {} has an active account {} with number of sign ins {}",
        user_1.username, user_1.email, user_1.number_of_sign_ins, user_1.active
    );

    let user_2 = User {
        email: String::from("xyz@gmail.com"),
        ..user_1
    };

    println!("user 2 is {:#?}", user_2);

    struct Color(u8, u8, u8);
    fn set_bg_color(color: Color) {
        println!(
            "The background is set to color with R={}, G={}, B={}",
            color.0, color.1, color.2
        );
    }
    let red = Color(100, 0, 0);
    set_bg_color(red);
}

#[derive(Debug)]
struct Rectangle {
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

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nicklel,
    Dime,
    Quarter(UsState),
}

fn values_to_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("We have a penny");
            1
        }
        Coin::Nicklel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Got an Alaskan Quarter");
            25
        }
        Coin::Quarter(state) => {
            println!("This coin is from {:#?}", state);
            25
        }
    }
}
