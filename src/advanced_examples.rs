use crate::lang::data::Data;
use crate::lang::processor::{Processor, IntegerProcessor, StringProcessor};

pub fn run() {
    closures_example();
    generics_example();
    ownership_example();
    trait_example();
    lifetimes_example();
}

fn closures_example() {

    // Basic closure, note pipe syntax
    {
        let basic = || {
            println!("Hello world");
        };

        basic();
    }

    // Closure with arguments, I guess '|x: i32, y: i32|' is like '(x: i32, y: i32)' in normal c like function declaration
    // to me the pipes syntax seem little odd anyway, what do i know
    {
        let add = |x: i32, y: i32| x + y;
        let result = add(5, 7);
        println!("The result is: {}", result);
    }

    // Return a value from closure
    {
        let sum = |a: i32, b: i32| -> i32 {
            a + b
        };

        println!("2 + 2 = {}", sum(2, 2));
    }


    // Capture variable
    {
        let x = 10;

        let print_x = || println!("x is: {}", x);
        print_x();
    }

    // Mutating Captured Variables
    {
        let mut count = 0;
        let mut increment = || {
            count += 1;
            println!("count is: {}", count);
        };

        increment();
        increment();
    }

    //Closures as Function Parameters
    {
        fn apply<F>(f: F)
        where
            F: Fn(),
        {
            f();
        }

        let greet = || println!("Hello, world!");
        apply(greet);
    }

    // Returning Closures from Functions
    {
        fn create_closure() -> impl Fn(i32) -> i32 {
            let base = 2;
            move |x| x + base
        }

        let closure = create_closure();
        let result = closure(5);
        println!("The result is: {}", result);
    }

    // Closures with Lifetimes
    {
        let x = 10;
        let print_x = || println!("x is: {}", x);
        call_closure(&print_x);

        fn call_closure<'a, F>(f: &'a F)
        where
            F: Fn() + 'a,
        {
            f();
        }
    }

    // Closure with 'move' take ownership of String
    {
        let string_value = String::from("Hello World");

        let basic = move || {
            println!("closure own the string now: {}", string_value);
        };

        basic();

        // Uncommenting the line below will cause a compile-time error because `string_value` is no longer accessible
        //println!("This won't work now?: {}", string_value);
    }
}

fn generics_example() {
    println!("\nGenerics Example:");
    let vec: Vec<i32> = vec![1, 2, 3];
    let doubled_vec: Vec<i32> = vec.iter().map(|&x| x * 2).collect();
    println!("Doubled vector: {:?}", doubled_vec);
}

fn ownership_example() {

    /*{
        let i = 3; // Lifetime for `i` starts. ────────────────┐
        //                                                     │
        { //                                                   │
            let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
            //                                                ││
            println!("borrow1: {}", borrow1); //              ││
        } // `borrow1` ends. ─────────────────────────────────┘│
        //                                                     │
        //                                                     │
        { //                                                   │
            let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
            //                                                ││
            println!("borrow2: {}", borrow2); //              ││
        } // `borrow2` ends. ─────────────────────────────────┘│
        //                                                     │
    }   // Lifetime ends. ─────────────────────────────────────┘
     */

    // Function that takes two references and returns a reference
    // The lifetime 'a indicates that both input references and the output reference are valid for the same lifetime.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("hello");
    let string2 = "world";

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);

    /// - Next Example

    // Lifetime elision rule allows us to omit the lifetime parameter in simple cases
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);

    /// - Next Example

    // Struct with a reference that requires a lifetime annotation
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);

    /// - Next Example

    //Nested Scopes

    fn use_references<'a, 'b, 'c>(ra: &'a i32, rb: &'b i32, rc: &'c i32) {
        println!("a: {}, b: {}, c: {}", ra, rb, rc);
    }

    //lifetime 'a
    {
        let a = 1;

        //lifetime 'b
        {
            let b = 2;

            //lifetime 'c
            {
                let c = 3;
                // Calling the function with references to `a, `b, and `c
                use_references(&a, &b, &c);
            } // `c goes out of scope here
        } // `b goes out of scope here
    } // `a goes out of scope here
}

fn trait_example() {
    println!("\nTrait Example:");
    let int_processor = IntegerProcessor;
    int_processor.process(Data::Integer(10));

    let string_processor = StringProcessor;
    string_processor.process(Data::String("world".to_string()));
}

fn lifetimes_example() {
    println!("\nLifetimes Example:");
    fn borrow_data<'a>(data: &'a Data) {
        println!("Borrowed data: {:?}", data);
    }

    let data = Data::Integer(15);
    borrow_data(&data);
}