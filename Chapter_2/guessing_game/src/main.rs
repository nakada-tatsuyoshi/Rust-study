use std::io;

fn main() {
    println!("Guess the number!");

    println!("please input your guess.");

    let mut guess = String::new(); //String型の可変変数を作成した。

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}