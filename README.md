# Guessing Game
My first rust chapter of the book `The Rust Programming Language` using neovim as IDE. 

## How to run the App?
Just clone this repository and run the command `cargo run`.


The out should be equal to the following output: 
```
cargo run
   Compiling guessing_game v0.1.0 (/Users/Christian.Jung/project/rust/the_rust_book/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
guessing Game - 0.0.1
Try: 1
Please input your guess(1-100):
1
Too small!
Try: 2
Please input your guess(1-100):
80
Too small!
Try: 3
Please input your guess(1-100):
90
Too big!
Try: 4
Please input your guess(1-100):
86
Too big!
Try: 5
Please input your guess(1-100):
82
You won!
You need 5 tries


```


## What did I learn?
- Use NeoVim as IDE with some basic setup and a LSP for Dart
- Setup NeoVim with different windows
- Use Cargo to create the project
- Use Cargo to run and update the project
- Create a random number
- Read the input
- Handle the `Result` with a `match`
- Work with a basic loop
