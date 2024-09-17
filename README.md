### Starting Rust

  - For every input command
    ```rust
    io::stdin().read_line( &mut guess).expect("Expected a number");
    ```

    there needs to be `guess.clear()` statment to avoid any appending of future numbers to the buffer of `guess` after every iteration in the loop.

  - To label a loop you use `'<label-name>`, if you have loop in a loop.

### Ownership and Referencing
  - The ownership is defined with scope
  - You can reference with `&` like referencing a `String` literal as `&str`.
  - for a variable a
    ```rust
    let a = String::from("Hi");
    ```
    it can be referenced as `&a`
  - the ownership of `a` stays until when the immutable reference `&a` exists in the code

### Slicing
  - A string can be sliced as 
    ```rust
    println!("{}", &[0..2]);
    ```
    OR
    ```rust
    println!("{}", &[0..]);
    ```
    OR
    ```rust
    println!("{}", &[..]);
    ```
### Understadning structure with struct

  - You can create a struct with `struct` keyword
  - They are similar to tuples that can take different fields as keys and data types
    ```rust
    struct User {
      username: String,
      email: String,
      active: bool,
    };
    ```
  - There are Struct tuples like
    ```rust
    struct Color (u8, u8, u8);
    ```
    which can thre RGB values of type `u8`
  - An instance needs to be created as 
    ```rust
    let user:User = User {
      username: String::from("Advit"),
      email: String::from("abc@gmail.com")
    }
    ```
  - These can be destructred by dot notation as `user.username`
  - The data types can only be owned data types like `String` that have `Copy` trait unlike `&str`
  - It is good practice to pass reference of the struct rather than the actual varibale to avoid the function taking the ownership by using the `&` operator.
    ```rust
    let area = calculate_area(&rect);
    println!("{}", rect);
    ```
  - To use structs in print statements, you need to provide implementation before the instantiation of the struct like
    ```rust
    #[dervie(Debug)]
    struct Rectangle{
      width: u32,
      height: u32,
    }

    let rect = Rectangle {
      width: 32,
      height: 40,
    }
    println!("{:#?}", &rect);
    ```
  - Creating methods for structs can be done using `impl` keyword which implements a method for the respective struct
    ```rust
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
    ```
  - `can_hold()` method here is an associated method which can be called as:
    ```rust
    Rectangle::square(32);
    ```





