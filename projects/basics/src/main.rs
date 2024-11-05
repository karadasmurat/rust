enum Color {
    Red,
    Green,
    Blue,
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn pass_fail(score: i32) {
    if score < 60 {
        println!("Fail");
    } else {
        println!("Pass");
    }
}

fn letter_score(score: i32) {
    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else if score >= 60 {
        println!("D");
    } else {
        println!("F");
    }
}

fn count_down(mut n: i32) {
    // let mut number = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF!!!");
}

// Declares a mutable parameter x of type i32
// Increments the value of x within the function
fn increment(mut x: i32) {
    x += 1;
    println!("Value of x inside the function: {}", x);
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn takes_ownership(s: String) {
    // s comes into scope
    println!("{s}");
} // Here, s goes out of scope and `drop` is called.
  // The backing memory is freed.

// move its return value into the function that calls it
fn gives_ownership() -> String {
    let s = String::from("yours"); // s comes into scope
    s // s is returned and moves out to the calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(s: String) -> String {
    // s comes into scope
    s // s is returned and moves out to the calling function
}

fn makes_copy(val: i32) {
    // val comes into scope
    println!("{val}");
} // Here, val goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    // we have to return the s to the calling function so we can still use after the call
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn main() {
    let x = 5; // Integer type is inferred
    let y: f64 = 3.14; // Explicitly typed as a 64-bit floating-point number
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let c = 'z';
    let z: char = '🙂'; // with explicit type annotation
    println!("The value of z is: {z}");

    let s = "hello";

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const HOURS_IN_SECONDS: u32 = 60 * 60;

    // We create a tuple by writing a comma-separated list of values inside parentheses.
    let tup = (500, 6.4, 1);

    let scores = [75, 80, 90];
    let first = scores[0];
    println!("First Score: {first}");

    let sum = add(5, 10);
    println!("The sum is: {}", sum);

    pass_fail(50);
    letter_score(80);

    let score = 45;

    // Conditional Assignment:
    let status = if score < 60 { "Fail" } else { "Pass" };

    println!("The status is: {status}");

    count_down(3);

    let mut y = 5;
    increment(y);
    println!("Value of y after the function call: {}", y); // 5

    let f = celsius_to_fahrenheit(10.0);
    println!("fahrenheit {f}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}, world!"); // hello, world!
                              // println!("{s1}, world!"); // ERR borrow of moved value: `s1`

    let name = "Alice";
    let age = 30;
    println!("Hello, {}! You are {} years old.", name, age);

    let pi = 3.14159;
    println!("Pi to 3 decimal places: {:.3}", pi);

    // match expression
    let number = 3;

    match number {
        1 => println!("One!"),
        2 | 3 => println!("Two or three!"),
        _ => println!("Any other number!"),
    }

    // Looping through each element of a collection
    let fruits = ["Apple", "Banana", "Orange", "Mango"];

    for fruit in fruits {
        println!("{fruit}");
    }

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("return value moved: {s1}");

    let s = String::from("argument"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                        // s is no longer valid here

    // println!("s is no longer valid here: {s}") // ERR value borrowed after move

    let myval = 5; // x comes into scope
    makes_copy(x); // i32 is Copy, so it's okay to still use x afterward
    println!("it's okay to still use myval afterward: {myval}");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    let s3 = String::from("hello");
    let len3 = calculate_length_ref(&s3);
    println!("The length of '{s3}' is {len3}.");

    //Mutable references
    let mut x = 5;
    let y = &mut x;

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // // let r3 = &mut s; // PROBLEM
    // // println!("{r1}, {r2}, and {r3}");

    // String slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // loop
    for i in (90..100).step_by(3) {
        println!("{i}");
    }

    count_down(3);

    let c = Color::Green;
    print_color(c);

    // Structs
    let user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    tuple_basics();
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("The color is RED!"),
        Color::Green => println!("The color is GREEN!"),
        Color::Blue => println!("The color is BLUE!"),
    }
}

fn tuple_basics() {
    println!("Tuple Basics");
    println!("------------");

    // Tuples are a simple way to group multiple values together, often of different types.
    let data = (1, "hello", 3.14);

    // use pattern matching to destructure a tuple
    let (x, y, z) = data;

    println!("{}", y); // hello

    // indexing
    let x = data.0;
    println!("{}", x); // 1
}
