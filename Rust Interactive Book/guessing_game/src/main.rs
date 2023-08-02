// If you expected this to be readable
// FUCK YOU
// I'm just writing so i memorize what each thing does and figure out the vibes of the language

use rand::Rng;
use std::cmp::Ordering;
use std::io; // Importing the standard library, io from within it
             // At this point i realized that these imports work based on types, io, Rng and Ordering might be considered types, and use brings them into scope. Then you can use functions related to them

fn main() {
    // Commands are the same as usual
    println!("Guess the number!"); // prints are the usual println, but with a ! for some reason

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng specifies that the random number generated will be in the same thread as the main program.
                                                               // gen_range specifies the range of possible numbers that can be generated
                                                               // this doesn't need error handling because it doesn't have error behavior?

    loop {
        // Generic while(true)
        println!("Please input your guess."); // println is a macro, ! might mean that

        let mut guess = String::new(); // Variables are by nature solid, they're basically constants, to have normal
                                       //variables you have to say they're mutable
                                       // Let is the standard way to declare variables
                                       // It's better to always take the input as a string and convert it later?
                                       // I wonder how rust checks for the size of the input and things like that

        io::stdin() // In calling a function from a library, if you do not import that function itself, you have to reference the library
            // stdin is technically a function, but it behaves like a handle, or like a pipe that's open for input
            .read_line(&mut guess) //these are methods. & means it's a reference. References are something like variables, but that interact differently with threads and functions. They can also be mutable or immutable
            .expect("Failed to read line"); // except is a mandatory method that handles the result of functions. Result is a type of it's own, a sort of enum, that can be ok or err. It's basically seeing errors are variables. If you don't handle this value the compiler will get mad at you.

        let guess: u32 = match guess.trim().parse() {
            // except automatically crashes the program, so instead we use match to specify the behavior of the error
            Ok(num) => num, // num is whatever parse returns inside Ok
            Err(_) => continue, // the _ is a catchall. Continue makes the cycle repeat itself emideatly
                                // This way, if you want to treat different errors appropriately, i think you change the _ to whatever you want and add more Err() clauses
        }; // This is the standard way to handle errors i think
           // this shadows the variable guess, such that it's new version becomes the true one
           // u32 is a type of number (type of int in particular). Later rust also infers secret number is a u32 too because it's compared to guess
           // trim erases the white spaces before and after guess (the enter gets saved as a white space at the end of guess)
           // parse converts the string to a number. It can fail, so an except is necessary

        println!("You guessed: {guess}"); // This is the way to print variables, if you want to print a compound expression, you do ("{}", expression)

        match guess.cmp(&secret_number) {
            // Match is like a broader switch case in that it matches an argument, but to an expression instead of to another argument
            Ordering::Less => println!("Too small!"), // cmp is the usual compare. Make sure both variables are of the same type
            Ordering::Greater => println!("Too big!"), // matche's expressions are rigid in structure which depends of the first expression (i don't really know where you can look it up, but good luck...)
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
