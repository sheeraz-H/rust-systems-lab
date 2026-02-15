fn main() {
    let is_true:bool = true;

    if is_true {
        println!("The number is  even")
    }else { 
        println!("The given number is not even!")
    }
    println!("Hello, sheeraz-osain!");


    // -----variables and mutability-----
    // rust variables are immurable by default

    let x = 5; // immutable
    println!("x = {}", x);

    let mut y = 10; //mutable
    println!("y = {}", y);
    y = 20;
    println!("y after change = {}", y);


    // mut -allows variable to change
    // {} -placeholder for values in println!

    // key concept: Rust wants you to think before changing things, making coe safter.

    // -----Data-Types------
    //  -Rust has static typing, meaining every variable has a type.

    // common types:

    let a: i32 = 10; //32-bit integer
    let b: f64 = 3.14; //64-bit float
    let c: bool = true; //boolean
    let d: char = 'R'; //single character
    let e: &str = "Rust"; //sring slice

    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);



    // -Rust can infer types, but explicit types help beginners.
    // -concept: Type saftey -> prvents wrong operations (like adding number to string).


    // -------Shadowing--------
    // you can reuse the same variable name but change its value/type

    let x = 5;
    let x = x + 1; // shadowing old x
    let x = x * 2;
    println!("x = {}", x);

    // safer than mut sometimes
    // allows transforamtion without mutating

    // ---------Fucntions----------
    // functions let you organize code:

    greet("Rust");
    let sum = addd(5, 7);
    println!("Sum = {}!", sum);


    // -----------Control Flow-----------

    // -if Else

    let number = 10;

    if number > 5 {
        println!("Greter than 5!");

    }else {
        println!("5 or less!");
    }


    //  ---------looooops-------
    let mut count = 0;

    while count < 5 {
        println!("count = {}", count);
        count += 1;
    }

    //////0rrrrrrr
    
    for i in 0..5 {
        println!("i = {}", i);
    }

    // Loops are similar to other languages, but for is safe and iterator-based.


    // --------------Ownership & Borrowing(Rust's core principle)

    // Rust's more unique concept.

    // -every value has one owner
    // -when owner goes out of scope -> value is dropped.
    // -Borrowing allows safe references

    let s1 = String::from("Hello");
    let s2 = &s1; // borrowing, s1 still owns it

    println!("s1 = {}, s2 = {}", s1, s2);

    // ---banefits -prevents memory leaks
    // --- prevents dangling pointers

    // Next step: learn mutable borrowing and move semantics.


    // ------------structs (Custom data Types)

    let user1 = User {
        username: String:: from("Kavus"),
        active: true,
    };

    println!("User: {}", user1.username);

    // Like classes without methods (methods come later with impl)

    // -----------Enums & Pattern Matching---------

    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Good up!"),
        Direction::Down => println!("Good down!"),
        Direction::Left => println!("Good left!"),
        Direction::Right => println!("Good right!"),

    }



}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn addd(a: i32, b: i32) -> i32{
    a + b // last expression is return value
}

// -> i32 → specifies return type

// No return needed for the last expression

// &str → string slice, a reference (important for ownership)


struct User {
    username: String,
    active: bool,
}


enum Direction {
    Up,
    Down,
    Left,
    Right
}