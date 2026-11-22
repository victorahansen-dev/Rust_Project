My Rust Learning Notes

Writing down what I've learnt so far while learning Rust.

Variables
Constants
rust
const MAX_POINTS: u32 = 100_000;

const declares a constant, which is a value that cannot be changed after being declared.
Constants must always have an explicitly defined data type (in this case u32).
Constants are immutable and cannot be declared using mut.
Mutable variables
rust
// Mutable variables can be changed after being declared.
let mut x = 5;

println!("x is {}", x); // Prints 5

x = 6;
println!("x is {}", x); // Prints 6

Variables declared using let are immutable by default.
Adding mut allows the variable to be modified.
Without mut, trying to assign a new value to the variable will cause a compiler error.
{} is a formatting placeholder that gets replaced by the value supplied to println!.
Shadowing
rust
let y = 5;
println!("y is {}", y); // Prints 5

let y = y + 1; // Creates a new variable with the value 6.
let y = y * 2; // Creates another new variable with the value 12.

println!("y is {}", y); // Prints 12.

Shadowing allows you to declare a new variable using the same name as an existing variable.
Each let declaration creates a new variable rather than modifying the existing one.
The new variable can use the value of the previous variable.
Shadowing allows you to transform values without needing mut.
Unlike mutability, shadowing also allows you to change the data type of a variable.
Data Types

Rust is a statically typed language, meaning variable types must be known at compile time.

Data types are divided into two main categories: scalar types and compound types.

Scalar types

Scalar types represent a single value.

Rust has four primary scalar types:

Integers
Floating-point numbers
Booleans
Characters
Integers
i32: A signed 32-bit integer. Can represent both positive and negative numbers.
u32: An unsigned 32-bit integer. Can only represent non-negative numbers.
Rust defaults to i32 when an integer type cannot otherwise be inferred.
Floating-point numbers
f32: A 32-bit floating-point number.
f64: A 64-bit floating-point number. This is the default floating-point type.
Booleans
bool represents either true or false.
Characters
char represents a single Unicode scalar value.
Characters are written using single quotation marks.
A char is not necessarily one visible character, since some visible symbols consist of multiple Unicode values.
rust
// Examples of scalar types.

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
rust
// A tuple groups multiple values together.
// Unlike arrays, tuples can contain different data types.

let person: (&str, u32, u32) = ("Maya", 29, 50000);

println!(
    "{} is {} years old and has a salary of {}",
    person.0, person.1, person.2
);

Tuples are declared using parentheses ().
Each element can have a different data type.
The types are specified in the same order as the values.
Individual elements can be accessed using dot notation and their index, starting at 0.
Tuples have a fixed length.

In this example:

person.0 accesses the name.
person.1 accesses the age.
person.2 accesses the salary.
Arrays
rust
// Arrays group multiple values of the same data type together.
// Arrays have a fixed length.

let scores: [i32; 3] = [90, 85, 100];

println!("First score is: {}", scores[0]);

Arrays are declared using square brackets [].
All elements must have the same data type.
[i32; 3] means the array contains three signed 32-bit integers.
Elements are accessed using their index, starting at 0.
Trying to access an index outside the array's bounds will cause a runtime panic when using normal indexing.
Functions

Functions allow you to organize code into reusable blocks.

Rust uses the fn keyword to declare a function.

rust
fn main() {
    greet("Victor");
    println!("{}", add(10, 20));
}

// Every parameter in a function must have an explicitly defined type.
fn greet(name: &str) {
    println!("Hello {}", name);
}

// Both parameters are defined as i32.
// -> i32 specifies that the function returns an i32.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

Return values
Functions can return values using the -> syntax.
The final expression in a function automatically becomes its return value if it does not end with a semicolon.
Expressions evaluate to a value, whereas statements perform an action without returning a value.
Adding a semicolon to the final expression turns it into a statement, meaning it no longer returns that value.

For example:

rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Expression: automatically returned.
}


You can also explicitly return a value using the return keyword.

Control Flow

Control flow allows programs to make decisions based on conditions.

If / else statements
rust
let number = 6;

// Check whether the remainder after dividing by 2 is zero.
if number % 2 == 0 {
    println!("The number is even!");
} else {
    println!("The number is odd!");
}

Everything inside an if or else block must be wrapped in curly braces {}.
% is the remainder operator.
== checks whether two values are equal.
If the condition evaluates to true, the if block executes.
Otherwise, the else block executes.
Conditions in Rust must evaluate to a Boolean (true or false).

In this example, dividing 6 by 2 leaves a remainder of 0, meaning the number is even.

Using if statements as expressions
rust
let number = 6;

// The if statement returns a value depending on the condition.
let label = if number % 2 == 0 {
    "even"
} else {
    "odd"
};

println!("{}", label);

Rust allows if statements to return values.
If the condition is true, "even" is returned.
Otherwise, "odd" is returned.
The returned value is assigned to label.
Both branches must return compatible data types.
Loops

Loops allow code to execute repeatedly.

Rust has three main types of loops:

loop
while
for
Loop
rust
// A loop runs indefinitely until a break statement is reached.

let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 5 {
        // Break terminates the loop.
        // It can also return a value from the loop.
        break counter * 2;
    }
};

println!("Loop result: {}", result);

loop creates an infinite loop.
counter must be mutable because its value changes during each iteration.
break terminates the loop.
break can also return a value from the loop.
In this example, when counter reaches 5, the loop terminates and returns 5 * 2.

The returned value is assigned to result, meaning it contains 10.

While loop
rust
let mut n = 3;

// The loop runs as long as n is not equal to zero.
while n != 0 {
    println!("{}", n);

    n -= 1; // Subtracts one from the current value of n.
}

println!("Liftoff!");

A while loop continues executing as long as its condition evaluates to true.
The condition is checked before every iteration.
Once n reaches 0, the condition becomes false and the loop terminates.

In this example, the output counts down from 3 to 1, followed by Liftoff!.

Ownership

Ownership is one of Rust's most important concepts.

It allows Rust to manage memory without requiring a garbage collector.

Ownership follows three main rules:

Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value is dropped and its memory can be freed.
Stack and heap
Stack: Stores data with a known, fixed size. Allocation and deallocation are generally very fast.
Heap: Stores data that can have a variable size or whose size isn't known at compile time. Memory is managed dynamically.

For example:

rust
let x = 5; // Integer with a known, fixed size.

let s = String::from("Hello"); // Growable String.


A String stores its contents in heap memory, while its pointer, length and capacity are stored in the String value itself.

Moving ownership
rust
fn main() {
    let s1 = String::from("Hello");

    let s2 = s1; // Ownership moves from s1 to s2.

    // println!("{}", s1); // This would cause a compiler error.
    println!("{}", s2);
}

String::from("Hello") creates an owned, growable string.
let s2 = s1 moves ownership from s1 to s2.
After the move, s1 is no longer valid.
s2 is now the owner of the string.

Rust prevents both variables from independently owning the same heap allocation. This helps prevent problems such as double-free errors.

Cloning
rust
fn main() {
    let s1 = String::from("Hello");

    let s2 = s1.clone(); // Creates an independent copy.

    println!("s1 = {}, s2 = {}", s1, s2);
}

.clone() creates a separate copy of the string.
Both variables now own their own strings.
Changing one string does not affect the other.
Cloning can involve allocating memory and copying data, so it can be more expensive than moving ownership.
Borrowing and References

Borrowing allows you to use a value without taking ownership of it.

A reference provides access to a value owned by another variable.

References are created using the & operator.

String vs &str

These two types are closely related but serve different purposes.

String is an owned, growable string.
&str is a borrowed string slice.
String owns its contents, whereas &str provides access to string data without owning it.
rust
let s1 = String::from("Hello");

let s2: &str = "World";

let s3: &str = &s1;

Immutable references
rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// The function borrows the string instead of taking ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
}

&s1 passes a reference to the string instead of transferring ownership.
s: &String means the function accepts a reference to a String.
The function can access the string without becoming its owner.
s.len() returns the length of the string as a usize.
Since ownership was never transferred, s1 remains valid in main.

This is useful when passing data into functions without giving up ownership.

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

Example of something that causes a compiler error:

rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // Error: cannot borrow s mutably more than once.

println!("{}, {}", r1, r2);


These restrictions prevent conflicting access to the same data.

Structs

Structs allow you to group related data together under one custom data type.

Unlike tuples, structs give each value a named field, making the data easier to understand and access.

Defining a struct
rust
// derive Debug automatically implements debug formatting support.

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

    // Access individual fields using dot notation.
    println!(
        "{} is {} years old and is active: {}",
        user1.username,
        user1.age,
        user1.active
    );

    // Debug formatting using {:?}.
    println!("{:?}", user1);

    // Calling a method attached to User.
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

Enums and Pattern Matching

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

        // Placeholder: the trapezoid calculation has not been implemented.
        Shape::Trapezoid(_, _, _) => 0.0,
    }
}

match shape checks which variant the enum contains.
Each arm handles a particular variant.
The values inside the variant can be extracted and used in calculations.
_ is a wildcard pattern that ignores a value.
Rust requires all possible variants to be covered in a match expression, unless a wildcard handles the remaining possibilities.
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

Option, Result and Error Handling

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
    // Handle both possible outcomes.
    match divide(10.0, 2.0) {
        Ok(value) => println!("The result is: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // Test another division, this time using negative zero.
    // In floating-point arithmetic, -0.0 == 0.0 evaluates to true.
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

Extra Notes
Macros

The exclamation mark (!) after certain Rust commands indicates that they are macros rather than regular functions.

Examples include:

rust
println!("Hello!");
format!("Hello!");
vec![1, 2, 3];

A macro is a way to generate code rather than simply execute a regular function.
Macros allow more flexible code generation and can accept patterns of input that ordinary functions cannot.
println! is an example of a macro that accepts formatting placeholders and multiple arguments.

For example:

rust
let name = "Victor";

println!("Hello {}, welcome to Rust!", name);


The macro processes the formatting syntax and generates the necessary code to print the formatted output.

The ! symbol is not exclusive to macros. For example, != is the comparison operator meaning "not equal to".
