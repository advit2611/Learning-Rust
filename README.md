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




