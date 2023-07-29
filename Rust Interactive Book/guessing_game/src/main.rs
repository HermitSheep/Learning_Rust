use std::io;    // Importing the standard library, io from within it
use rand::Rng;
use std::cmp::Ordering; // At this point i realized that these imports work based on types, io, Rng and Ordering might be considered types, and use brings them into scope. Then you can use functions related to them

fn main() {     // Commands are the same as usual
    println!("Guess the number!");  // prints are the usual println, but with a ! for some reason

    let secret_number = rand::thread_rng().gen_range(1..=100);  // thread_rng specifies that the random number generated will be in the same thread as the main program.
    // gen_range specifies the range of possible numbers that can be generated
    // this doesn't need error handling because it doesn't have error behavior?

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");   // println is a macro, ! might mean that

    let mut guess = String::new();  // Variables are by nature solid, they're basically constants, to have normal
                                    //variables you have to say they're mutable
                                    // Let is the standard way to declare variables

    io::stdin()     // In calling a function from a library, if you do not import that function itself, you have to reference the library
                    // stdin is technically a function, but it behaves like a handle, or like a pipe that's open for input
        .read_line(&mut guess)  //these are methods. & means it's a reference. References are something like variables, but that interact differently with threads and functions. They can also be mutable or immutable
        .expect("Failed to read line");     // except is a mandatory method that handles the result of functions. Result is a type of it's own, a sort of enum, that can be ok or err. It's basically seeing errors are variables. If you don't handle this value the compiler will get mad at you.

    println!("You guessed: {guess}");   // This is the way to print variables, if you want to print a compound expression, you do ("{}", expression)
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}