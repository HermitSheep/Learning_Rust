// This is the chapter on variables

// constants ARE_LIKE_THIS
// vareables are_like_this


const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    // The difference between constants and immutable variables is that constants can be declared everywhere, have to have their type defined immediately, they have to be associated to a fixed value, never an expression, and that they can't ever be mutable. Constants are good for holding values that are used all over the program. Some teachers say that all numbers that appear in the code should actually be constants, so you can change them more easily (you, the user, not the programmer, think of it as a game where you want the player to be able to cheat with the stats). (this for all the values except for 1's and 0's like in cycles of if statements, of course).

fn main() {
    let mut x = 5;  // Variables are by default immutable, which means that after getting a value once, it can't ever change, or get a new one
    // You add mut if you want to make the variable actually variable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    {
        let x *= 2;
        println!("The value of x is: {x}");
        // This is shadowing. It's basically reusing the name of a variable. It's useful for when you want to change the type of a variable, in a way, since you're not changing it's value, but creating anew variable with the same name instead. (like changing it from a sting to the number of characters in the string)
        // In this case, when the {} closes, x will return to being what it was outside this scope.
    }

    // Ints range from i8-128 (signed) to u8-128 (unsigned), to declare them you do:
    let num: i32; //or, = 42
    // Ints always have to be powers of 2, and the number represents the maximum size of the variable, signed go from 2^(n-1) to 2^n -1, while unsigned go from 0 to 2^n -1.

    // You can also use isize and usize instead, and the compiler will pick the ints size according to the computer's architecture. (So the int conforms the the computer the program is running on)

    // In rust, if the number grows bigger than the int, it wraps back around (and gives you a panic if you're debugging)

    // Floats are the same logic, instead they are f8-128 (most used are 32 and 64, with single and double precision)

    //Ahh, and booleans also exist, you don't have to import them like you do in c. The key words are bool, true and false.

    // Char's work the same way they do in c, but they also allow emotes and such. They use Unicode Scalar Value instead of ASCII, so you can even use letters other alphabets.

    // Tuple: You declare it like this:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Holy moly, your tuples can now hold different types, damn. (I wouldn't stretch this beyond numbers tho)
    // You can also pull things out of a tuple like so:
    let (a, s, d) = tup;
    let f = tup.2;
    // If you use an empty tuple, that is called a unit. It represents an empty value, or return type. (maybe use this instead of null)
    // ! You can't change a tuples size after it's been created
    // ? You can have different types in the same tuple

    // Arrays: You can declare them like this:
    let array = [1, 2, 3, 4, 5];
    let array2: [i32; 5];
    let array3 = [3; 5]; //this is an array with size 5 filled with 3's
    // Arrays have to have the same type in them and you can't change their size after they've been declared.
    // Arrays are more useful when you want to allocate data on the stack, or when you know exactly the number of/elements you want in a list.
    
    // To access an array's elements you do:
    let first = array[0];
    // If you try to access an element past the end of the array either the compiler will catch it, or the program will (with a panic)
    // ! You can't change an arrays size after it's been created
    // ! You can't have different types in the same array

    // There are also vectors, when i get to them in the book, if i still remember this, i'll put them here.
}