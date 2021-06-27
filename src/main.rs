use std::{f32::consts::PI, usize};

// unsafe Rust
// safe Rust

// *** borrow checker *** => prevent memory leak, ensure app is memory safe (performance & security)
// memory leak => data racing (more than one party consuming same data)
fn main() {
    println!("Hello, world!");
    println!("{}", area(Shape::Circle { radius: 1.0 }));

    // Borrow checker sample
    // Languages e.g. C that needs manual memory allocation
    // let x: i32;
    // malloc(x, 32);
    // x = 3;
    // free(x);

    // Similar to puppeteer scraping where browser page are also handled manually
    // let page = pupputeer.open()
    // // scraping
    // page.close()

    // In Rust, we have borrow checker
    /*
    The borrow checker is the component in the Rust compiler that enforces data
    ownership rules, and it enforces these to prevent data races
    */
    // Rules enforced by Rust borrow checker
    // (0) everthing value is immutable by default
    // (1) each variable can only be consumed (transferrence of ownership) once
    // (2) a variable can be borrowed immutably multiple times
    // (3) a variable can only be borrowed once mutably
    // mutable XOR alias

    // Sample for rule (1)
    let s: String = "hello world".to_string();

    /*
    let y: String = s;
    let z: String = s; // Error: use of moved value: `s`
    // To solve the error above
    let y: String = s.clone();
    let z: String = s;
    */

    /*
    print_string_1(s);
    // let y: String = s.clone(); // Error: borrow of moved value: `s`
    let z: String = s; // Error: use of moved value: `s`
    // To solve the error above
    print_string_2(&s);
    let z: String = s;
    */

    // Sample for rule (1) & (2)
    // let spongebob: String = "hello world".to_string();
    // print_string_2(&spongebob); // 1st solution: borrow using reference
    // print_string_1(spongebob.clone()); // 2nd solution: borrow using clone
    // print_string_1(spongebob);
    // print_string_1(spongebob);
    // print_string_1(spongebob);
    // print_string_1(spongebob);
    // let patrick: String = spongebob;

    // Sample for rule (3)
    let mut spongebob = "hello world".to_string();
    // append_krab_1(spongebob);
    // append_krab_2(&mut spongebob, &mut spongebob); // Error: cannot borrow `spongebob` as mutable more than once at a time

    // In Rust, no need to perform memory allocation/garbage collection manually
    // malloc/free
    // open/close

    let spongebob: String = "Lucifer Morningstar".to_string();
    let mut spongebob1: String = spongebob.clone();
    // Immutability
    // - return modified result
    let krab_1 = append_krab_1(spongebob);
    let krab_11 = append_krab_11(&mut spongebob1);
    println!("{}", krab_1);
    println!("{}", krab_11);
    println!("{}", spongebob1);

    println!("{}", factorial(2));
    println!("{}", factorial(3));
}

// Default function return type: Unit type = ()
// a type with only constructor
// unit type = null, undefined in Typescript

// lifetime ~= type
/*
Reference: https://blog.logrocket.com/understanding-lifetimes-in-rust/

Lifetimes are what the Rust compiler uses to keep track of how long references are valid for.
Checking references is one of the borrow checker’s main responsibilities. Lifetimes help the
borrow checker ensure that you never have invalid references.
*/
fn print_string_1<'s>(s: &'s String) {
    println!("{}", s)
    // free(a) < -- NO THIS LINE
    // Should be collected in the origin function
}

fn print_string_2(s: &String) {
    println!("{}", s)
}

// Sample for rule (0)
// mutable
fn append_krab_1(mut input: String) -> String {
    input.push('k');
    input
}

fn append_krab_11(input: &mut String) -> &mut String {
    input.push('k');
    input.push('x');
    input
}

// &mut = mutable reference
// &mut String = mutable reference of String
fn append_krab_2(a: &mut String, b: &mut String) {
    a.push('k');
    b.push('k');
}

// *** Data types ***
// usize, i8, i16, i32, i64
// u32, i32
// i32
// usize - unsigned integer with size depends on the system architecture
// String => variable length string, &str => fixed length string

// Sum types - discriminated unions
enum Shape {
    Circle { radius: f32 },
    Rectangle { side: f32, length: f32 },
    Triangle { base: f32, height: f32 },
    TwoShapes(Box<Shape>, Box<Shape>),
}
// Total number of possible combinations = 3
// Sum of variants
enum Boolean {
    True,
    False,
    Null,
}

enum Color {
    R,
    G,
    B,
}

// Product types
// struct = keyed product type
struct Human {
    id: String,
    name: String,
}
// tuple = jey-less product type
type Human2 = (String, String); // tuple
                                // Total number of possible combinations = 4
struct Human3 {
    id: Boolean,   // 2
    name: Boolean, // 2
}
fn possible_humans() {
    let humans = vec![
        Human3 {
            id: Boolean::True,
            name: Boolean::True,
        },
        Human3 {
            id: Boolean::True,
            name: Boolean::False,
        },
        Human3 {
            id: Boolean::False,
            name: Boolean::True,
        },
        Human3 {
            id: Boolean::False,
            name: Boolean::False,
        },
    ];
}

// Total number of possible combinations = 12
struct Human12 {
    is_crazy: Boolean, // 2
    is_tall: Boolean,  // 2
    face_color: Color, // 3
}

// Total number of possible combinations = 12
type Human22 = (Boolean, Boolean, Color);

// 1. Normal nested pattern matching
fn is_red_crazy_short_human_normal(human: Human12) -> bool {
    match human {
        Human12 {
            is_crazy,
            is_tall,
            face_color,
        } => match face_color {
            Color::R => match is_crazy {
                Boolean::True => match is_tall {
                    Boolean::False => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
    }
}
// 2. Better nested pattern matching
fn is_red_crazy_short_human_better(human: Human12) -> bool {
    match human {
        Human12 {
            is_crazy: Boolean::True,
            is_tall: Boolean::False,
            face_color: Color::R,
        } => true,
        _ => false,
    }
}

fn is_red_crazy_short_human22_better(human: Human22) -> bool {
    match human {
        (Boolean::True, Boolean::False, Color::R) => true,
        _ => false,
    }
}

// *** Pattern matching ***
// 1. Normal pattern matching
fn area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle { radius } => PI * radius * radius,
        Shape::Rectangle { side, length } => side * length,
        Shape::Triangle { base, height } => base * height / 2.0,
        Shape::TwoShapes(a, b) => area(*a) + area(*b),
    }
}

// 2. Nested pattern matching
// Normal
fn is_two_circle_normal(shape: Shape) -> bool {
    match shape {
        Shape::TwoShapes(a, b) => {
            match *a /* Box type variable needs to be deferenced when usage */ {
            Shape::Circle { .. } /* '..' is to ignore the parameters */ => match *b {
                Shape::Circle { .. } => true,
                _ => false,
            },
            _ => false,
        }
        }
        _ => false,
    }
}

// Better
fn is_two_circle_better(shape: Shape) -> bool {
    match shape {
        Shape::TwoShapes(a, b) => match (*a, *b) {
            (Shape::Circle { .. }, Shape::Circle { .. }) => true,
            _ => false,
        },
        _ => false,
    }
}

// Another nested pattern matching example
// circle = big, if radius > 10
// Normal
fn is_two_big_circles_normal(shape: Shape) -> bool {
    match shape {
        Shape::TwoShapes(a, b) => match (*a, *b) {
            (Shape::Circle { radius: radius_a }, Shape::Circle { radius: radius_b }) => {
                if radius_a > 10.0 && radius_b > 10.0 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        },
        _ => false,
    }
}

// Better
fn is_two_big_circles_better(shape: Shape) -> bool {
    match shape {
        Shape::TwoShapes(a, b) => match (*a, *b) {
            (Shape::Circle { radius: radius_a }, Shape::Circle { radius: radius_b })
                if radius_a > 10.0 && radius_b > 10.0 =>
            {
                true
            }
            _ => false,
        },
        _ => false,
    }
}

// ***Data structure design problem***
// IMPORTANT: Enforce as much invariants (checking/validation) in the data type as possible
// - eliminate the needs to validate too much things in program logic (possibly forgot to validate something for all usages)
// and might possibly validate the wrong things so why not let the compiler to help do the validation i.e. type checking
// - make the data safer to use

// Bad: struct with a lot of optional properties
struct BadShape {
    radius: Option<f32>, // 2
    side: Option<f32>,   // 2
    base: Option<f32>,   // 2
}

enum List<T> {
    Nil, // 8-bytes
    Cons {
        head: T,            // 32
        tail: Box<List<T>>, // 32
    },
}

// stack = fixed/initial memory (known at compile time)
// heap = dynamic memory (known only at runtime)
fn foo() {
    // malloc(mylist, 64 + 8)
    let mylist: List<u32> = List::Nil;
    let x = Box::new(3);
    let y = *x + 2;

    // 1. it does not prevent malconstruction
    // malloc(shape, 96)
    let shape = BadShape {
        radius: None,
        side: Some(2.0),
        base: Some(3.0),
    };
    let shape2 = BadShape {
        radius: Some(9.0),
        side: Some(2.0),
        base: Some(3.0),
    };

    // Solution for 1.: With correct data type, it prevents excess or lack of properties
    let shape0 = Shape::Circle { radius: 32.0 };
    // let shape0 = Shape::Circle {  }; // missing structure fields
    // let shape0 = Shape::Circle { radius: 32.0, base: 2.0  }; // no such field

    // In Typescript, it is quite tedious to validate/parse nested properties
    // if shape.radius.a.b.c.d !== undefined {
    //     shape.radius.a.b.c.d * 2
    // }
    // In Rust can use its pattern matching for validation/parsing more easily

    // Almost everything in Rust is an expression (value)
    let x = match shape.radius {
        Some(radius) => radius * 2.0,
        None => 0.0,
    };
    let hello = if true { "hello" } else { "world" };
    let x = {
        let a = 2;
        let b = 3;
        a + b
    };

    // Mapping from current form to a new form
    let x_ = shape
        .radius
        .map(|radius| radius * 2.0)
        .unwrap_or_else(|| 0.0);
    // Typescript equivalent
    // let x = shape.radius.map(radius => radius * 2.0);
}

// 2. it doesn't help with writing functions
// Need to validate a lot of things (possibly forgot to validate something for all usages)
// And might possibly validate the wrong things
fn area_1(shape: BadShape) -> f32 {
    match shape {
        BadShape {
            radius: Some(radius),
            side: None,
            base: None,
        } => {
            // this is a circle
            radius * radius * PI
        }
        BadShape {
            radius: None,
            side: Some(side),
            base: Some(base),
        } => {
            // rectangle
            base * side / 2.0
        }
        _ => panic!("this is a bad shape"),
    }
}

// Solution for 2.: With correct data type, it with help with validation (precise and correct)
fn area_12(shape: Shape) -> f32 {
    match shape {
        Shape::Circle { radius } => radius * radius * PI,
        Shape::Rectangle { side, length } => side * length,
        Shape::Triangle { base, height } => base * height / 2.0,
        _ => panic!(),
    }
}

// Looping
// factorial(10) = 10 * 9 * ... * 1
// factorial(3) = 3 * 2 * 1
// factorial(2) = 2 * 1
fn factorial(mut n: usize) -> usize {
    let mut result = 1;
    loop {
        if n <= 1 {
            break;
        } else {
            result = result * n;
            n = n - 1;
        }
    }
    result
}

fn factorial2(mut n: usize) -> usize {
    let mut temp = 1;
    let result = loop {
        if n <= 1 {
            break temp;
        } else {
            temp = temp * n;
            n = n - 1;
        }
    };
    result
}

fn factorial3(mut n: usize) -> usize {
    let mut temp = 1;
    loop {
        if n <= 1 {
            break temp;
        } else {
            temp = temp * n;
            n = n - 1;
        }
    }
}

struct Account {
    balance: f32,
}

// Explicit error handling - using Result type (Sum type)
fn withdraw_account(account: Account, amount: f32) -> Result<f32, String> {
    if account.balance < amount {
        // In Rust, no throwing error
        //  throw new Error("Insufficient balance");

        // Rust version
        // Result::Err("Insufficient balance".to_string())
        Err("Insufficient balance".to_string())
    } else {
        //  db.update(account);
        //  return

        // Rust version
        // Result::Ok(account.balance - amount)
        Ok(account.balance - amount)
    }
}

fn debit_account(account: &Account, amount: f32) -> Result<f32, String> {
    todo!("Not implemented yet")
}

fn transfer_amount_1(
    from_account: Account,
    to_account: Account,
    amount: f32,
) -> Result<f32, String> {
    let result = withdraw_account(from_account, amount);

    // pattern matching hell
    match result {
        Ok(_) => match debit_account(&to_account, amount) {
            Ok(_) => Ok(to_account.balance + amount),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

fn transfer_amount_2(
    from_account: Account,
    to_account: Account,
    amount: f32,
) -> Result<f32, String> {
    let result = withdraw_account(from_account, amount)?;
    debit_account(&to_account, amount)?;
    Ok(to_account.balance + amount)
}
