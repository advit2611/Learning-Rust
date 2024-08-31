- For every input command
  ```rust
  io::stdin().read_line( &mut guess).expect("Expected a number");
  ```

  there needs to be `guess.clear()` statment to avoid any appending of future numbers to the buffer of `guess` after every iteration in the loop.

- To label a loop you use `'<label-name>`, if you have loop in a loop.
