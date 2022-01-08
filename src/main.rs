// For Input from the console
use std::io;
// For the random number generator
use rand::Rng;
// For comparing the guesses
use std::cmp::Ordering;

// function to generate a random number between 1 and 100
fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn main() {
    println!("Guess the Number!");

    // Generate a random number between 1 and 100
    let secret_number = generate_random_number();

    loop {
        println!("Enter a number: ");

        // instantiate a new variable of type String for input
        // reads from the console, stdin stores it in the buffer "guess"
        // expects to throw an error if failure occurs in reading the input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // type casting to u32
        let guess :u32 = guess.trim().parse::<u32>().expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less =>    println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

