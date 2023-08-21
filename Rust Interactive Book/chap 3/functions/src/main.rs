fn main() {
    // SIKE, we'll talk a bit about expressions as well as functions. The rust interactive book goes on and on about statements and expressions, but i think things are simpler than that.

    // What is needed to be explained about expressions is compound expressions, when you create a variable based on another variable you also just created, for example. Basically you have to separate things with {}, so like:
    let y = {
        let x = 3;
        x + 1
    };

    // When it comes to functions, they work like this:
    fn function(variable: i32) -> i32 {
        // something something
        // Notice how the return type comes after the functions declaration
        
        // About the way to return things, if you have a very simple function, you can do something like:
        variable += 1
        // And this returns variable. If you put a ; after it it breaks the function because it'll stop returning anything. It goes from an expression to a statement. Basically, expressions return things, and statements don't.
        // In all honesty, it's cleaner if you just use the return key word instead of looking for a lacking ;
    }
}
