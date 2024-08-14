use crate::examples::greet::*;
use crate::examples::guess_the_number;
use crate::examples::passing_string::*;
use crate::models::cars::Cars::*;
use crate::models::simple::Simple;
use crate::types::type_example::*;
use crate::utils::aliases::{Byte, Double, Int};
use crate::utils::static_utils::*;

pub fn run() {
    greetings_example();
    string_representations();
    enums_example();
    guessing_game_example();
    string_passing_example();
    static_utils_example();
    type_example();
    tuple_example();
    scopes_example();
    shadowing_example();
}

fn greetings_example() {
    // Simple greeting function
    greet_number(100);

    // Using `String` type for heap allocation
    let my_greeting: String = "World".to_string();
    greet_string(my_greeting);

    // Using `String::from` for heap allocation
    let alternative_string = String::from("this works too");
    println!("{}", alternative_string.as_str());

    // Working with string slices (`&str`) and static lifetimes
    let my_greeting_str;
    {
        my_greeting_str = "Hello World";
        greet_str(my_greeting_str);
    }

    // Implicit &str literal
    let test_string = "Hello";
    println!("{}", test_string);

    // Using `&str` to format and return a `String`
    let result: String = greet_format(my_greeting_str);
    println!("result: {}", result);

    // Creating and printing a struct instance
    let simple = Simple {
        name: "Hello".to_string(),
        id: 10,
    };
    println!("simple: {}", simple);
}

fn string_representations() {
    // Different types of string representations
    let person_name_string = String::from("Donald Mallard");
    let person_name_slice = &person_name_string; // Borrow as &String
    let person_name_slice2 = person_name_string.as_str(); // Convert to &str explicitly

    // String concatenation using format macro
    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);
}

fn enums_example() {
    // Using a simple enum from cars.rs
    let car = Volkswagen;

    // Match statement to handle different enum variants
    match car {
        Volkswagen => println!("Volkswagen"),
        Ford => println!("Ford"),
        Toyota => println!("Toyota"),
        Kia => println!("Kia"),
        Hyundai => println!("Hyundai"),
    }
}

fn guessing_game_example() {
    // Using GuessTheNumber struct from guess_the_number module
    let guess_the_number = guess_the_number::GuessTheNumber {};
    guess_the_number.guess();
}

fn string_passing_example() {
    let passing_string = PassingString {
        value: String::from("Hello"),
    };

    // Passing as a reference
    println!("{}", passing_string.value());

    // Cloning a string
    let clone_string: String = passing_string.clone();
    println!("{}", clone_string);

    // Demonstrating ownership and cloning
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    let next = original.clone(); // Clone the original string
    println!("{}", original);

    original = "what about now".to_string();
    println!("next {}", next);
    println!("original {}", original);
}

fn static_utils_example() {
    println!("\nStatic Utils Example:");
    let concatenated = StaticUtils::concatenate("hello", " world");
    println!("Concatenated: {}", concatenated);
}

fn type_example() {
    println!("\nType Example:");
    let mut example = TypeExample::default();
    example.set_boolean(true);
    println!("Boolean value: {}", example.boolean());
}

fn tuple_example() {
    // Declared tuple
    let tuple: (Int, Double, Byte) = (500, 6.4, 1);

    // Access a tuple element directly by using a period (.) followed by the index
    let int_value = tuple.0;

    let double_value = tuple.1;

    let byte_value = tuple.2;

    // pattern matching a tuple value,
    let (x, y, z) = tuple;

    println!("The value of x is: {x}. The value of y is:{y}. The value of z is:{z}.");
}

fn scopes_example() {
    let scope_test = "outer scope";
    println!("{}", scope_test);

    // inner scope different variable from outer scope
    {
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }

    println!("{}", scope_test);
}

fn shadowing_example() {
    // int 1
    let shadowing_test = 1; //once declare shadowing_test again (line below) it will overwrite the original variable

    // overwrite int 1 variable (above) with formated string referencing original value using string interpolation
    let shadowing_test = format!("the variable above is int and it value {shadowing_test}");

    // will print out the string
    println!("{}", shadowing_test);
}