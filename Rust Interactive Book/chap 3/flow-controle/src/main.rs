fn main() {
    // This is basically if's and what not.
    // Here are the new things about if's:
    // You can't do 
    let number: bool = true;

    if number {
        println!("does this work?")
    }
    // if number isn't a boolean, which simply awesome. Seriously.
    // Also, you can't not have {}, even for single line if's, which is also a bit cleaner i guess.

    // I don't know if you could do this before, but i'd never seen it, so now you know you can do:
    let something = if number {3} else {5}
    // This makes something 3, if number is true, and 5, if number is false. Sorry for the terrible naming, but either way, this is kinda cool.
    // You just have to make sure the if and else have compatible types, one can't be a number and the other a string


    // Now onto loops!
    // To begin with, you have a loop, which is the same as a while(true)
    println!("Please, please spare my wife and children, I beg of you, please")
    loop {
        println!("please")
    }
    // You can return a value from a loop using break as if it were a return
    let result = loop {
        let counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    // (honestly this looks quite a bit weird, having break return something, but i could see it cleaning up very specific situations)
    
    // Super neat thing, loops can have labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // Notice here, it's going to break the outer loop using a label
                break 'counting_up; 
            }
            remaining -= 1;
        }
        count += 1;
    }
    // This is cool as hell isn't it?

    // Whiles are now written like this:
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    // And fors look like this:
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // Or like this:
    for number in (1..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    // The first example is better than using a while, because you're guaranteed not to go beyond the bounds of the array
    // The second is a more conventional for loop. It uses range (x..y) as it's bounds, rev() just reverses the order of the range. (i assume x has to always be bigger than y). Range is in the standard library.
}
