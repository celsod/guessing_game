use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 10!");  //This is how you start a comment

    let secret_number = rand::thread_rng().gen_range(1..=10);
    //This line above creates a secret number variable that is random between 1 and 10

    //println!("The secret number is: {}", secret_number); //This is similar to the fstring in python
    //uncomment the line above it you want to see the secret number

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); //Created a new variable called guess and is able to be changed

        io::stdin()
            .read_line(&mut guess) //calls the read_line method for input to the program
            //the &mut guess is the argument to pass the input into
            .expect("Failed to read the line"); //error code if something goes wrong

        let guess: u32 = match guess.trim().parse() { //trim removes whitespace around the input and parse converts the string to u32
            Ok(num) => num, //if guess is a number move to the next step in the program
            Err(_) => continue, //if guess is not a number, repeat the loop
        };
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number){ //compare function between guess and secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => { //function to end the program when the correct number is selected
                println!("Correct!");
                break;}
            Ordering::Greater => println!("Too large!"),
        }
    }
}
