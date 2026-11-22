Writing down what I've learnt so far while learning Rust.

1. Variables
Constants

Constants are values that cannot be changed after being declared. Unlike regular variables, constants must always have an explicitly defined data type.

rust
const MAX_POINTS: u32 = 100_000;

const declares a constant.
MAX_POINTS is the name of the constant.
u32 specifies its data type.
Constants are immutable and cannot be declared using mut.
Mutable variables

Variables declared using let are immutable by default. Adding mut makes them mutable, meaning their values can be changed after declaration.

rust
let mut x = 5;

println!("x is {}", x); // Prints 5

x = 6;
println!("x is {}", x); // Prints 6

mut allows the variable to be modified.
Without mut, trying to assign a new value to x would cause a compiler error.
{} is a formatting placeholder that gets replaced by the value supplied to println!.
Shadowing

Shadowing allows you to declare a new variable using the same name as an existing variable.

Unlike mutability, shadowing creates a new variable rather than modifying the existing one.

rust
let y = 5;
println!("y is {}", y); // Prints 5

let y = y + 1; // New variable, with the value 6
let y = y * 2; // Another new variable, with the value 12

println!("y is {}", y); // Prints 12

Each let declaration creates a new variable.
The new variable can use the value of the previous variable.
Shadowing allows you to transform a value without needing mut.
Shadowing can also be used to change the data type of a variable, something mutability alone cannot do.
2. Data Types

Rust is a statically typed language, meaning variable types must be known at compile time.

Data types are generally divided into two categories: scalar types and compound types.

Scalar types

Scalar types represent a single value. Rust has four primary scalar types:

Integers
Floating-point numbers
Booleans
Characters
Integers

Integers represent whole numbers without decimal places.

i32: A signed 32-bit integer. Can represent both positive and negative numbers.
u32: An unsigned 32-bit integer. Can only represent non-negative numbers.

Rust defaults to i32 when an integer type cannot otherwise be inferred.

Floating-point numbers

Floating-point numbers represent numbers with decimal places.

f32: A 32-bit floating-point number.
f64: A 64-bit floating-point number. This is the default floating-point type.
Booleans

Booleans represent either true or false.

Characters

The char type represents a single Unicode character. Characters are written using single quotation marks.

rust
let age: u32 = 18;
let num: i32 = -100;
let temperature: f64 = 36.6;
let is_active: bool = true;
let grade: char = 'A';

println!(
    "Age: {}, Number: {}, Temperature: {}, Active: {}, Grade: {}",
    age, num, temperature, is_active, grade
);

Compound types

Compound types allow multiple values to be grouped together.

Rust has two primitive compound types:

Tuples
Arrays
Tuples

Tuples group multiple values together into one compound type.

Unlike arrays, tuples can contain values of different data types.

rust
let person: (&str, u32, u32) = ("Maya", 29, 50000);

println!(
    "{} is {} years old and has a salary of {}",
    person.0, person.1, person.2
);

Tuples are declared using parentheses ().
Each element has its own data type.
The types are specified in the same order as the values.
Individual elements can be accessed using dot notation and their index, starting at 0.
Tuples have a fixed length.

In this example:

person.0 accesses the name.
person.1 accesses the age.
person.2 accesses the salary.
Arrays

Arrays group multiple values of the same data type together.

Unlike vectors, arrays have a fixed length.

rust
let scores: [i32; 3] = [90, 85, 100];

println!("First score is: {}", scores[0]);

Arrays are declared using square brackets [].
All elements must have the same data type.
[i32; 3] means the array contains three signed 32-bit integers.
Elements are accessed using their index, starting at 0.
Trying to access an index outside the array's bounds will cause a runtime panic when using normal indexing.
3. Functions

Functions allow you to organize code into reusable blocks.

Rust uses the fn keyword to declare a function.

rust
fn main() {
    greet("Victor");
    println!("{}", add(10, 20));
}

fn greet(name: &str) {
    println!("Hello {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

Function parameters

Function parameters must have explicitly defined data types.

For example:

rust
fn greet(name: &str) {
    println!("Hello {}", name);
}


Here, name is a parameter with the type &str, meaning it accepts a string slice reference.

The add function takes two parameters, both of type i32.

Return values

Functions can return values using the -> syntax.

rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

-> i32 specifies that the function returns an i32.
a + b is an expression.
Expressions evaluate to a value.
The final expression in a function is automatically returned if it does not end with a semicolon.
Using a semicolon turns the expression into a statement, meaning it no longer returns that value.

You can also return values explicitly using the return keyword.

4. Control Flow

Control flow allows programs to make decisions based on conditions.

If / else statements

if and else statements allow code to execute depending on whether a condition evaluates to true or false.

rust
let number = 6;

// Check whether the remainder after dividing by 2 is zero.
if number % 2 == 0 {
    println!("The number is even!");
} else {
    println!("The number is odd!");
}

% is the remainder operator.
== checks whether two values are equal.
If the condition evaluates to true, the if block executes.
Otherwise, the else block executes.
Rust requires the conditions of if statements to evaluate to a Boolean.
Using if statements as expressions

In Rust, if statements can also return values.

rust
let number = 6;

let label = if number % 2 == 0 {
    "even"
} else {
    "odd"
};

println!("{}", label);

If the condition is true, "even" is returned.
Otherwise, "odd" is returned.
The returned value is assigned to label.
Both branches must return compatible types.

This is useful when assigning values based on conditions without needing to write a separate function.

5. Loops

Loops allow code to execute repeatedly.

Rust has three main types of loops:

loop
while
for
Loop

A loop runs indefinitely until it encounters a break statement.

rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 5 {
        break counter * 2;
    }
};

println!("Loop result: {}", result);

loop creates an infinite loop.
counter must be mutable because its value changes during each iteration.
break terminates the loop.
break can also return a value from the loop.
In this example, when counter reaches 5, the loop terminates and returns 5 * 2.

The returned value is assigned to result, which means it will contain 10.

While loop

A while loop continues executing as long as its condition evaluates to true.

rust
let mut n = 3;

while n != 0 {
    println!("{}", n);
    n -= 1;
}

println!("Liftoff!");

The loop checks whether n is different from 0.
If the condition is true, the loop executes.
n -= 1 subtracts one from the current value of n.
Once n reaches 0, the condition becomes false and the loop terminates.

In this example, the output counts down from 3 to 1, followed by Liftoff!.

6. Ownership

Ownership is one of Rust's most important concepts.

It allows Rust to manage memory without requiring a garbage collector.

Every value in Rust has an owner, and ownership follows three main rules:

Each value has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value is dropped and its memory can be freed.
Moving ownership
rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}", s1); // This would cause a compiler error.
    println!("{}", s2);
}

String::from("Hello") creates an owned, growable string.
let s2 = s1 moves ownership from s1 to s2.
After the move, s1 is no longer valid.
s2 is now the owner of the string.

Rust prevents both variables from independently owning the same heap allocation. This helps prevent problems such as double-free errors.

Cloning

If you want to create an independent copy instead of moving ownership, you can use .clone().

rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

.clone() creates a separate copy of the string.
Both variables now own their own strings.
Changing one string does not affect the other.
Cloning can involve allocating memory and copying data, so it can be more expensive than moving ownership.
7. Borrowing and References

Borrowing allows you to use a value without taking ownership of it.

A reference provides access to a value owned by another variable.

References are created using the & operator.

Immutable references
rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

&s1 passes a reference to the string instead of transferring ownership.
s: &String means the function accepts a reference to a String.
The function can access the string without becoming its owner.
s.len() returns the length of the string as a usize.
Since ownership was never transferred, s1 remains valid in main.

This is useful when you want to pass data into functions without giving up ownership.

Mutable references

References can also be mutable, allowing a function to modify the original value.

rust
fn main() {
    let mut s = String::from("hello");

    add_exclamation(&mut s);

    println!("{}", s);
}

fn add_exclamation(text: &mut String) {
    text.push_str("!");
}

mut makes the original string mutable.
&mut s creates a mutable reference.
text: &mut String allows the function to modify the borrowed string.
.push_str("!") appends text to the existing string.
The function modifies the original value without taking ownership.
Borrowing rules

Rust enforces borrowing rules to prevent data races and other memory safety problems.

The main rules are:

You can have multiple immutable references to a value at the same time.
You can have only one mutable reference to a value at a time.
You cannot have mutable and immutable references to the same value active at the same time.

For example, this causes a compiler error:

rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // Error: cannot borrow s mutably more than once.

println!("{}, {}", r1, r2);


These restrictions help prevent multiple parts of a program from modifying the same data simultaneously in unsafe ways.

8. Structs

Structs allow you to group related data together under one custom data type.

Unlike tuples, structs give each value a named field, making the data easier to understand and access.

Defining a struct
rust
#[derive(Debug)]
struct User {
    username: String,
    age: u32,
    active: bool,
}

struct declares a new custom data type.
User is the name of the struct.
Each field has a name and a data type.
#[derive(Debug)] tells Rust to automatically implement the Debug trait, allowing the struct to be printed using debug formatting.
Creating an instance
rust
fn main() {
    let user1 = User {
        username: String::from("Victor"),
        age: 18,
        active: true,
    };

    println!(
        "{} is {} years old and is active: {}",
        user1.username,
        user1.age,
        user1.active
    );

    println!("{:?}", user1);

    println!("{}", user1.describe());
}

An instance is created by providing values for each field.
Individual fields can be accessed using dot notation.
{:?} uses debug formatting, which is available because we derived Debug.
Methods and impl

You can attach methods to a struct using an impl block.

rust
impl User {
    fn describe(&self) -> String {
        format!("{} (active: {})", self.username, self.active)
    }
}

impl defines methods associated with a particular type.
describe is a method belonging to User.
&self is shorthand for borrowing the current instance immutably.
It allows the method to access the fields of the struct without taking ownership.
format! creates a formatted String instead of printing directly to the terminal.

When calling user1.describe(), Rust automatically passes a reference to user1 as self.

9. Enums and Pattern Matching

Enums allow you to define a type that can represent one of several possible variants.

Unlike structs, which group fields together, enums represent a value that can be one of several alternatives.

Defining enums
rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}


Each value of TrafficLight can represent only one of its variants at a time.

Enums can also contain data:

rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Trapezoid(f64, f64, f64),
}

Circle contains one f64.
Rectangle contains two f64 values.
Trapezoid contains three f64 values.

This allows different variants of the same enum to hold different amounts of data.

Pattern matching with match

The match expression allows you to compare a value against different patterns and execute the corresponding code.

rust
fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Trapezoid(_, _, _) => 0.0,
    }
}

match shape checks which variant the enum contains.
Each arm handles a particular variant.
The values inside the variant can be extracted and used in calculations.
_ is a wildcard pattern that ignores a value.
Rust requires all possible variants to be covered in a match expression, unless a wildcard handles the remaining possibilities.

The trapezoid calculation is currently a placeholder returning 0.0.

Using the enums
rust
fn main() {
    let light = TrafficLight::Red;

    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(3.0, 4.0);
    let t = Shape::Trapezoid(1.0, 2.0, 3.0);

    println!("Circle area: {}", area(c));
    println!("Rectangle area: {}", area(r));
    println!("Trapezoid area: {}", area(t));

    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}


The match expression checks the value of light and executes the matching arm.

In this example, only the Red arm executes, printing Stop.

10. Option, Result and Error Handling

Rust uses enums to represent situations where a value might be missing or an operation might fail.

Two important enums for this are Option and Result.

Option

Option<T> represents a value that may or may not exist.

It has two possible variants:

Some(T): Contains a value.
None: Represents the absence of a value.
rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Victor"))
    } else {
        None
    }
}

fn main() {
    let result = find_user(1);

    match result {
        Some(name) => println!("Found {}", name),
        None => println!("No user found"),
    }
}

Option<String> means the function can return either a String or no value.
Some(String::from("Victor")) returns a string wrapped inside Some.
None indicates that no value was found.
match is used to handle both possibilities.

This is useful when a value might not exist, such as searching for a user in a database.

Result

Result<T, E> is used when an operation can either succeed or fail.

It has two possible variants:

Ok(T): Represents success and contains the successful value.
Err(E): Represents failure and contains information about the error.

Unlike Option, which represents the possibility of a missing value, Result can provide information about why an operation failed.

For example:

rust
// Result<f64, String> means:
// - Success returns an f64.
// - Failure returns a String containing an error message.

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        // Return Err when the divisor is zero.
        Err(String::from("cannot divide by zero"))
    } else {
        // Wrap the successful result inside Ok.
        Ok(a / b)
    }
}

fn main() {
    // Handle the successful and unsuccessful outcomes.
    match divide(10.0, 2.0) {
        Ok(value) => println!("The result is: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // Test another division, this time using negative zero.
    // In floating-point arithmetic, -0.0 == 0.0 is true.
    match divide(10.0, -0.0) {
        Ok(value) => println!("The result is: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

Result<f64, String> specifies the types of both possible outcomes.
Ok(value) contains the successful division result.
Err(error) contains the error message.
match handles the different variants.
Rust's exhaustive pattern matching ensures that every possible variant is handled unless a wildcard pattern is used.

One advantage of Result is that the function does not decide what should happen when an error occurs. Instead, it returns the error to the caller, allowing the caller to decide how to handle it.

11. Extra Notes
Macros

The exclamation mark (!) after certain Rust commands indicates that they are macros rather than regular functions.

Examples include:

rust
println!("Hello!");
format!("Hello!");
vec![1, 2, 3];


A macro is a way to generate code rather than simply execute a regular function.

Macros are useful because they allow more flexible code generation and can accept patterns of input that ordinary functions cannot.

For example, println! accepts formatting placeholders and multiple arguments:

rust
let name = "Victor";
println!("Hello {}, welcome to Rust!", name);


The macro processes the formatting syntax and generates the necessary code to print the formatted output.

The ! symbol is not exclusive to macros. For example, != is the comparison operator meaning "not equal to".
