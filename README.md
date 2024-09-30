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

### Leraning Enums
  - Enums can be initialized with keyword `enum`
  - These can store multiple values with different types
    ```rust
    enum IpAddressKind{
      Ipv4,
      Ipv6,
    }
    ```
  - You can add type constraints to these values with the following syntax
    ```rust
    enum IpAddressKind{
      Ipv4(u8, u8, u8, u8),
      Ipv6(String),
    }
    ```
  - These enums can be stored as a datatype in structs
    ```rust
    struct network{
      A: String,
      Routing: IpAddressKind
    }
    ```
  - Storing a value of particular enum as:
    ```rust
    let ip_type = IpAddressKind::Ipv4(0, 0, 0, 0)
    ```

### Packages and crates
  - There can be at most one `lib.rs` file in a project
  - `lib.rs` can never be run
  - all modules and variables instantiated needed to be declared public with `pub` keyword.
  - The functions can be imported using `use` keyword and referencing root name of the project
    ```rust
    use hello_cargo::authenticate;
    use hello_cargo::Credentials;
    ```
  - A file can be decalared a module with `mod` keyword
  - There can be two types of refrences:
    - Absolute reference
      ```rust
      crate::authenticate::login();
      ```
    - Relative reference
      ```rust
      super::connect_to_database::connect();
      ```

### Vectors
  - Vectors are like lists or arrays that can hold multiple values of same data type
    ```rust
    let vec = Vec::new();
    ```
    or 
    ```rust
    let vec = vec![1, 2, 3];
    ```
  - Vectors can hold different data types by instantiating a enum and using it to store the values
  - Values of a vector can be changed by derefrencing
    ```rust
    let mut vec = vec![1, 1, 1, 3, 5, 8]
    let third_value = &mut vec[2];
    *third_value = 2
    println!("The correct fibonacci is {:?}", vec);
    ```
### Strings
  - Strings are a wrapper to vectors
  - ASCII strings can be sliced/indexed over by:
    ```rust
    let s = "Hello";
    println!("First character", &s[0..1]);
    println!("Second to last character", &s[1..-1]);
    ```
  - The `+` operator is an implementation like `add(self, other: &str)` function, the first operand is borrowed and second can be referenced
    ```rust
    let s3 = s1 + &s2;
    ```
  - To not borrow anything
    ```rust
    let s4 = format!("{s1}{s2}")
    ```
  - Strings are stored as bytes and non-ASCII characters can have 2 bytes for one character. As rust indexes over bytes, non-ASCII strings can make the code panic. 
  - To slice over a non-ASCII string
    ```rust
    let s5: String = String::from("नमस्ते").chars().take(2).collect();
    println!("First two characters of s5 non-ASCII string is {}", s5);
    ```
  - `.chars()` converts the String type to characters
  - `.take()` takes first n character in the string
  - `.collect()` takes the string and stores in the variable
  - The data type needs to be explicitly specified for `collect()` function

### HashMaps
  - These can be instantiated by 
    ```rust
    let scores: Hashmap<String, i32> = Hashmap::new();
    scores.insert(String::from("blue"), 10);
    ```
  - You can store different data types by declaring a enum
  - for inserting and updating a value at the same time
    ```rust
    scores.insert(String::from("blue"), scores.get(&String::from("blue")).copied().unwrap_or(0) + 5);
    ```
    Here, `copied()` can be used to avoid dereferencing the `0` value sa generally required with `&0`
  - `scores.get(&key)` returns an `Option<{something}>` that can be implemented with `Some()` and `None`
  - as the `.get()` returns a reference to the value `&score`, it can be used to update the existing value by getting the actual value with `*score`
  - To return the current value for a key or insert a default value if key does not exist you can use 
    ```rust
    let count = word_count.entry(word).or_insert(0);
    *count += 1
    ```
    it needs to dereferenced with `*count` to update
    




