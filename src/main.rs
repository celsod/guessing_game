use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 10!");  //This is how you start a comment

    let secret_number = rand::thread_rng().gen_range(1..=10);
    //This line above creates a secret number variable that is random between 1 and 10

    println!("The secret number is: {}", secret_number); //This is similar to the fstring in python

    println!("Please input your guess:");

    let mut guess = String::new(); //Created a new variable called guess and is able to be changed

    io::stdin()
        .read_line(&mut guess) //calls the read_line method for input to the program
        //the &mut guess is the argument to pass the input into
        .expect("Failed to read the line");

    println!("You guessed: {guess}");
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Correct!"),
        Ordering::Greater => println!("Too large!"),
    }
}
