fn main() {
    // VARIABLES AND MUTABILITY
    // - variables are immutable by default: This means cannot be changed or assigned a new value
    // - To make variable mutable, add the mut keyword

    let mut x = 5;
    println!("The value of x is {x}");

    x = 7;
    println!("The value of x is {x}");

    // CONSTANTS
    // - Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
    // - First, you aren’t allowed to use mut with constants
    // - Constants aren’t just immutable by default—they’re always immutable
    // - You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is {THREE_HOURS_IN_SECONDS}");

    // SHADOWING
    // - Redeclaring a variable with the declaration keywords

    let y = 5;
    println!("The value of y is {y}");

    let y = 6;
    println!("The value of y is {y}");

}
