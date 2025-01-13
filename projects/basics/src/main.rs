mod audio;
mod exercises; // declare the module
mod geometry; // declare the module // since no lib.rs, we declare module
use audio::filters::low_pass_filter; // shortcut for the path

// use basics::pass_fail; // import lib function

// use std::collections::Vec;

use std::collections::HashMap;

enum Color {
    Red,
    Green,
    Blue,
}

// enum with data
enum Message {
    Quit,
    Move(i32, i32), // Tuple-like variant
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// user parameter takes ownership of the value
fn get_signin_count_move(user: User) {
    println!(
        "user: {} sign in count: {}",
        user.username, user.sign_in_count
    )
}

fn get_signin_count_borrow(user: &User) {
    println!(
        "user: {} sign in count: {}",
        user.username, user.sign_in_count
    )
}

fn increment_signin_move(mut user: User) -> User {
    user.sign_in_count += 1;

    // return modified value
    user
}

// Mutable Reference
fn increment_signin_mutref(user: &mut User) {
    user.sign_in_count += 1;

    // no return
}

fn add(x: i32, y: i32) -> i32 {
    x + y
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

//
fn increment_err(x: i32) {
    // x += 1; // ERR cannot assign to immutable argument
    println!("Value of x inside the function: {}", x);
}

// Declares a mutable parameter x of type i32
// Increments the value of x within the function
fn increment(mut x: i32) {
    x += 1;
    println!("Value of x inside the function: {}", x);
}

fn modify_arg_err(point: Point) {
    // point.x = 10; // This will cause a compile-time error
    println!("Point coordinates: ({}, {})", point.x, point.y);
}

fn modify_arg(mut point: Point) {
    point.x = 10;
    println!("Point coordinates: ({}, {})", point.x, point.y);
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

fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None // Division by zero is not allowed
    } else {
        Some(x / y)
    }
}

fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}

fn type_basics() {
    println!("Type Basics");
    println!("-----------");

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

    let scores = [75, 80, 90];
    let first = scores[0];
    println!("First Score: {first}");

    // mutability
    let mut x = 0; // x is immutable here
                   // increment_err(x); // ERR

    increment(x); // This creates a copy of 'x' and passes it to the function (primitive)

    // Remember: To modify the value of x in the main function, you need to pass a reference to x to the increment function:
    println!("x after the function call: {}", x); // x remains 0

    tuple_basics();
    string_basics();
}

fn tuple_basics() {
    println!("Tuple Basics");
    println!("------------");

    // Tuples are a simple way to group multiple values together, often of different types.
    // We create a tuple by writing a comma-separated list of values inside parentheses.
    let data = (1, "hello", 3.14);

    // Using a tuple to represent a user
    // A struct can be more readable and maintainable since named fields in a struct provide better self-documentation.
    let user: (bool, String, String, u64) = (
        true,
        String::from("user1"),
        String::from("user1@example.com"),
        1,
    );

    // use pattern matching to destructure a tuple
    let (x, y, z) = data;

    println!("{}", y); // hello

    // indexing
    let x = data.0;
    println!("{}", x); // 1

    // a function that returns a tuple
    let (p, a) = perimeter_and_area(5.0, 10.0);
    println!("w: 5.0 h: 10.0 perimeter: {} area: {}", p, a);
}

fn array_basics() {
    println!("Array Basics");
    println!("------------");

    // declare an array and initialize it with five integer values: 30, 70, 45, 85, and 35
    // The size of the array (5 elements) is inferred by the compiler
    let scores1 = [30, 70, 45, 85, 35];

    let scores2: [i32; 4] = [70, 80, 90, 50];

    // Access the first element (index 0)
    let first = scores1[0];

    // get the length of an array using the len() method
    let len = scores1.len();

    println!("First score: {} Number of Scores: {}", first, len);

    // Looping through each element of a collection
    let fruits = ["Apple", "Banana", "Orange", "Mango"];

    for fruit in fruits {
        println!("{fruit}");
    }

    // iterate over the array
    for s in scores1 {
        let res = if s < 50 { "Fail" } else { "Pass" };
        println!("{}: {}", s, res);
    }
}

fn vector_basics() {
    println!("Vector Basics");
    println!("-------------");

    // Vectors are implemented using generics
    // we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
    let v: Vec<i32> = Vec::new();

    // Using vec! macro
    // Create a vector with initial values - Rust will infer the type
    let v2 = vec![1, 2, 3];

    // to change its value, we need to make it mutable
    let mut v3: Vec<i32> = Vec::new();

    // Using the push method to add values to a vector
    v3.push(50);
    v3.push(60);

    // Using & and [] gives us a reference to the element at the index value.
    let first = v3[0];
    println!("First element: {}", first);

    // What if you try to use an index value outside the range of existing elements? panic!
    // When the get method is passed an index that is outside the vector, it returns None without panicking.
    // let does_not_exist1 = &v[100]; // panic
    let does_not_exist2 = v.get(100);

    // Using match to handle the Option<...>
    match does_not_exist2 {
        Some(x) => println!("Value at index 100 is: {}", x),
        None => println!("Index out of bounds."),
    }

    //
    let mut hogwarts = Vec::new();

    let harry = String::from("Harry");
    let ron = String::from("Ron");
    let hermione = String::from("Hermione");

    hogwarts.push(harry); // value moved here
    hogwarts.push(ron);
    hogwarts.push(hermione);

    // println!("{}", harry); // ERR value barrowed here after move

    println!("{:?}", hogwarts); // ["Harry", "Ron", "Hermione"]
}

fn hashmap_basics() {
    println!("Hashmap Basics");
    println!("-------------");

    // create an empty hash map
    let mut scores = HashMap::new();

    // insert elements
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // the existing value is updated with the new value.
    scores.insert(String::from("Blue"), 11);
    println!("{:?}", scores); // {"Blue": 11, "Yellow": 20}

    // insert a key only if it doesn't already exist
    scores.entry(String::from("Blue")).or_insert(12);
    scores.entry(String::from("Red")).or_insert(30);

    println!("{:?}", scores); // {"Yellow": 20, "Red": 30, "Blue": 11}

    // access elements
    // get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.
    let score_blue = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("{}", score_blue);

    // iterate over keys
    // &scores provides an immutable borrow of the HashMap, allowing you to iterate without modifying it.
    for key in scores.keys() {
        println!("key: {}", key);
    }

    // Iterate over HashMap
    // Using &scores in the for loop creates an iterator over the HashMap
    // Borrowing: After the loop, the scores variable would no longer be valid
    for (k, v) in &scores {
        println!("key: {}, value: {}", k, v);
    }
}

fn conditionals() {
    println!("Conditionals");
    println!("------------");

    let score = 45;

    // Conditional Assignment:
    let status = if score < 60 { "Fail" } else { "Pass" };

    println!("The status is: {status}");
}

fn struct_basics() {
    println!("Struct Basics");
    println!("-------------");

    // Structs
    let user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let rect = geometry::Rectangle { w: 5, h: 10 };

    // method, borrowed self
    // method can access the struct's data but cannot modify it.
    println!("Area: {}", rect.area());

    // To use a method with &mut self (which modifies the instance),
    // the instance itself must be declared as mutable.
    let mut rect2 = geometry::Rectangle { w: 1, h: 2 };

    // method, mutable borrowed self
    rect2.scale(2);
    println!("Scaled Area: {}", rect2.area());
}

fn string_basics() {
    println!("String Basics");
    println!("-------------");

    // create a new, empty String
    let mut s1 = String::new();

    // string literal
    let s2 = "string 2";

    // String::from and to_string do the same thing
    let s3 = "string 3".to_string();
    let s4 = s2.to_string();

    // Using the String::from function to create a String from a string literal
    let s5 = String::from("string 5");

    println!("{} {} {} {} {}", s1, s2, s3, s4, s5);

    // update a string
    let mut s6 = String::from("foo");
    s6.push_str("bar"); // foobar
    println!("{}", s6);

    // String slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    let s3 = String::from("hello");
    let len3 = calculate_length_ref(&s3);
    println!("The length of '{s3}' is {len3}.");
}

fn enum_basics() {
    println!("Enum Basics");
    println!("-----------");

    let c: Color = Color::Green;

    match c {
        Color::Red => println!("Variant: Color Red"),
        _ => println!("Some other color"),
    }

    let msg = Message::Move(10, 5);

    match msg {
        Message::Move(x, y) => {
            println!("Moving to x: {}, y: {}", x, y);
        }
        _ => {}
    }

    // Option<T> enum
    let result1 = divide(10.0, 2.0); // Some(5.0)
    let result2 = divide(10.0, 0.0); // None

    match result2 {
        Some(val) => println!("Result: {}", val),
        None => println!("Cannot divide by zero."),
    }

    //unwrap_or
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("x: {}", x.unwrap_or(0)); // Output: x: 5
    println!("y: {}", y.unwrap_or(10)); // Output: y: 10
}

fn reference_basics() {
    println!("Reference Basics");
    println!("----------------");

    let s = String::from("MK");
    let s_ref1 = &s;
    let s_ref2 = &s;

    assert!(s_ref1 == s_ref2);

    let s1 = String::from("hello");
    let s2 = &s1;

    print_type(&s2);
}

fn iterator_basics() {
    println!("Iterator Basics");
    println!("---------------");

    let values = vec![1, 2, 3];

    // create (mutable) iterator over the elements of the vector values
    // it yields references to the integers within the vector.
    let mut itr1 = values.iter();

    // The next() method on iterators attempts to return the next element in the iteration.
    // This inherently modifies the state of the iterator.
    assert_eq!(itr1.next(), Some(&1));

    assert_eq!(itr1.next(), Some(&2));
    assert_eq!(itr1.next(), Some(&3));
    assert_eq!(itr1.next(), None);

    // Iterator Adapters
    let scores = vec![71, 40, 60, 80, 90, 30, 85, 55];

    // map() transforms an existing iterator by applying a function to each element.
    // creates a new iterator that yields the transformed elements.
    // double each element
    let double: Vec<_> = scores.iter().map(|x| x * 2).collect();
    println!("Doubled scores: {:?}", double);

    // filter
    // *x: This dereferences the reference &x to get the actual integer value.
    let passes: Vec<_> = scores.iter().filter(|&x| *x >= 60).cloned().collect();
    // let passes: Vec<_> = scores.iter().filter(|x| x >= 60).collect();

    println!("{:?}", passes);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // consuming adapter sum
    let total: i32 = v1_iter.sum(); // 6
    println!("sum(): {}", total);

    let sum: i32 = passes.iter().sum(); // Explicitly specify the sum type
    let cnt = passes.len() as f64;
    let avg = sum as f64 / cnt;
    println!("{:?}", avg);
}

fn module_basics() {
    println!("Module Basics");
    println!("-------------");

    audio::generate_sine_wave();
    audio::filters::low_pass_filter(); // Access the function using full path
                                       // filters::low_pass_filter(); // use audio::filters;
    low_pass_filter(); // use audio::filters::low_pass_filter;
}

fn main() {
    let sum = add(5, 10);
    println!("The sum is: {}", sum);

    letter_score(80);

    count_down(3);

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

    let myval = 5; // myval comes into scope
    makes_copy(myval); // i32 is Copy, so it's okay to still use x afterward
    println!("it's okay to still use myval afterward: {myval}");

    //Mutable references
    let mut x = 5;
    let y = &mut x;

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // // let r3 = &mut s; // PROBLEM
    // // println!("{r1}, {r2}, and {r3}");

    count_down(3);

    // use a function defined in lib.rs
    // let res = pass_fail(40);

    // type_basics();
    // string_basics();
    // tuple_basics();
    // array_basics();
    // vector_basics();
    // hashmap_basics();
    // enum_basics();
    // conditionals();
    // loop_basics();
    // iterator_basics();
    // ownership_basics();
    // match_basics();
    // struct_basics();
    // reference_basics();
    module_basics();

    // run_exercises();
}

fn run_exercises() {
    let s = "Hello, world!";
    // exercises::strings::letter_frequencies(&s);
    // exercises::shorts::piggybank_2();
    exercises::shorts::bank_account_demo();
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("The color is RED!"),
        Color::Green => println!("The color is GREEN!"),
        Color::Blue => println!("The color is BLUE!"),
    }
}

fn match_basics() {
    println!("Match Expressions");
    println!("-----------------");

    let x = 100;
    let message = match x {
        // Multiple match patterns may be joined with the | operator
        0 | 1 => "not many",
        2..50 => "a few",   // end-exclusive
        50..=100 => "many", // end-inclusive
        _ => "lots",
    };

    println!("{x}, {message}"); // 100, many

    // match expression
    let number = 3;

    match number {
        1 => println!("One!"),
        2 | 3 => println!("Two or three!"),
        _ => println!("Any other number!"),
    }

    let c = Color::Green;
    print_color(c);
}

fn ownership_basics() {
    println!("Ownership");
    println!("---------");

    let a = 5; // An integer value 5 is created and assigned to the variable a.
    let b = a; // The value of a (which is 5) is moved to b.

    println!("b: {}", b); // b:5
    println!("a: {}", a); // a:5

    let x = 0;
    add_and_print(x);
    println!("after call> x: {}", x);

    // Struct
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    // get_signin_count_move(user1); // note that value moved here.
    // println!("user1 dropped, not accessible here! {}", user1.username); // ERR

    get_signin_count_borrow(&user1); // borrow
    println!("user1 accessible here! {}", user1.email);

    // Modify function argument
    // v1: ownership transfer
    let user_v2 = increment_signin_move(user1);
    println!("visits: {}", user_v2.sign_in_count); // visits: 2

    let mut user3 = User {
        active: true,
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        sign_in_count: 1,
    };
    // v2: mutable reference
    increment_signin_mutref(&mut user3);
    println!("visits: {}", user3.sign_in_count); // visits: 2
}

fn add_and_print(mut x: i32) {
    x += 1;
    println!("x == {}", x);
}

// calculate and return perimeter and area of a rectangle together
fn perimeter_and_area(w: f64, h: f64) -> (f64, f64) {
    (2.0 * (w + h), w * h)
}

fn loop_basics() {
    println!("Loop Basics");
    println!("-----------");

    // Iterate from 1 to 4 (exclusive of 5)
    for i in 1..5 {
        println!("{}", i); // Prints: 1 2 3 4
    }

    // Iterate from 1 to 5
    for i in 1..=5 {
        println!("{}", i); // Prints: 1 2 3 4 5
    }
}
