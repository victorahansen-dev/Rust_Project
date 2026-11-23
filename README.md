
# Rust Programming Notes

These are my notes on learning Rust, covering the fundamentals of the language, including variables, data types, functions, control flow, ownership and borrowing.

---

# 1. Variables

Variables are used to store information that can be accessed and used throughout a program.

There are three main things to understand when working with variables:

- **Immutable variables:** Cannot be changed after being assigned a value.
- **Mutable variables:** Can be changed after being assigned a value.
- **Constants:** Values that are always immutable and must have their data type explicitly defined.

## Constants

```rust
// const is always immutable - the data type must be specified

const MAX_POINTS: u32 = 100_000;
```

- `const` is used to declare a constant.
- `MAX_POINTS` is the name of the constant.
- `u32` defines the data type as an unsigned 32-bit integer.
- `100_000` is the value assigned to the constant.

**Important:**
- Constants cannot be changed after being declared.
- Their data type must always be specified.
- Constant names are conventionally written using uppercase letters and underscores.

## Immutable variables

```rust
// let creates an immutable variable by default
let x = 5;

println!("x is {}", x); // prints 5

// trying to change x here will cause an error
// this is because x was not declared as mutable
```

## Mutable variables

```rust
// mutable variable - allows the value to be changed after being set
let mut x = 5;

println!("x is {}", x); // prints 5

// since x is mutable, we can now change its value from 5 to 6
x = 6;

println!("x is {}", x); // prints 6
```

**Important:**
- `mut` allows an existing variable to be reassigned.
- Without `mut`, trying to change the value will cause a compiler error.
- Changing a mutable variable does not allow you to change its data type.

## Shadowing

Shadowing allows you to create a new variable using the same name as an existing variable.

Instead of changing the original variable, Rust creates a new variable that takes precedence over the previous one.

```rust
let y = 5;

println!("y is {}", y); // prints 5

// shadowing - creates a new variable using the same name
let y = y + 1; // new variable with the value 6

let y = y * 2; // another new variable using the previous value

println!("y is {}", y); // prints 12
```

**Important:**
- Shadowing creates a new variable rather than changing the existing one.
- This means you do not need to use `mut`.
- Shadowing also allows you to change the data type of a variable.

Example:

```rust
let spaces = "   "; // string slice

let spaces = spaces.len(); // new variable, now an unsigned integer (usize)
```

This works because the second `spaces` is a completely new variable.

---

# 2. Data Types

Rust is a statically typed language, meaning that every variable must have a known data type when the program is compiled.

Rust can often determine the data type automatically through type inference, but you can also specify it manually.

There are two main categories of data types:

- Scalar types
- Compound types

## 2.1 Scalar types

Scalars are single-value data types that include integers, floating-point numbers, Booleans and characters.

### Integers

Integers are whole numbers without decimal places.

- `i32` (signed) is the default integer type if no other type is specified.
- `i32` can contain both negative and positive numbers.
- `u32` (unsigned) can only contain non-negative numbers.
- The number after `i` or `u` represents the number of bits used.

```rust
// i32 is a signed 32-bit integer
let num: i32 = -100;

// u32 is an unsigned 32-bit integer
let age: u32 = 18;
```

Other integer types include `i8`, `i16`, `i64`, `i128`, `u8`, `u16`, `u64` and `u128`.

### Floating-point numbers

Floating-point numbers are used for numbers containing decimal places.

- `f32` is a 32-bit floating-point number.
- `f64` is a 64-bit floating-point number.
- `f64` is the default floating-point type.

```rust
let temperature: f64 = 36.6;
let height: f32 = 1.85;
```

### Booleans

Booleans are used when assigning a variable a value that is either true or false.

```rust
let is_active: bool = true;
let is_finished: bool = false;
```

### Characters

The `char` data type is used to represent a single Unicode character.

```rust
let grade: char = 'A';
let symbol: char = '!';
```

- Characters use single quotation marks.
- A `char` represents one Unicode character, not a whole word or sentence.
- Strings should be used when storing multiple characters.

### Using scalar types together

```rust
let age: u32 = 18;
let num: i32 = -100;
let temperature: f64 = 36.6;
let is_active: bool = true;
let grade: char = 'A';

println!(
    "Age: {}, Number: {}, Temperature: {}, Active: {}, Grade: {}",
    age, num, temperature, is_active, grade
);
```

The curly braces are placeholders that are replaced by the corresponding variables in the order they are provided.

---

## 2.2 Compound types

Compound types allow you to group multiple values together under one type.

Rust has two main primitive compound types:

- Tuples
- Arrays

### Tuples

A tuple groups different values together, even if they have different data types.

```rust
// tuple - groups different data types together
// each value inside the tuple must match its corresponding data type

let person: (&str, u32, u32) = ("Maya", 29, 50_000);
```

Here:
- `&str` represents the person's name.
- The first `u32` represents their age.
- The second `u32` represents their salary.

Values inside a tuple can be accessed using their position.

```rust
println!(
    "{} is {} years old and has a salary of {}",
    person.0, person.1, person.2
);
```

- `.0` accesses the first value.
- `.1` accesses the second value.
- `.2` accesses the third value.

Tuple positions start at zero.

You can also separate the values into individual variables:

```rust
let person = ("Maya", 29, 50_000);

let (name, age, salary) = person;

println!("{} is {} years old", name, age);
```

### Arrays

Arrays are used to store multiple values of the same data type.

Unlike vectors, arrays have a fixed size that cannot change after they have been created.

```rust
// array - stores multiple values of the same data type

// i32 specifies the data type of each value
// 3 specifies that the array contains three elements

let scores: [i32; 3] = [90, 85, 100];
```

Individual values can be accessed using their index.

```rust
println!("First score is: {}", scores[0]);
println!("Second score is: {}", scores[1]);
println!("Third score is: {}", scores[2]);
```

**Important:**
- Arrays use square brackets.
- All values must have the same data type.
- The number after the semicolon specifies the array's fixed length.
- Indexing starts at zero.
- Trying to access an index that does not exist will cause a runtime panic.

You can also use `.len()` to check how many elements an array contains.

```rust
println!("Array length: {}", scores.len());
```

---

# 3. Functions

Functions are reusable blocks of code that perform a specific task.

They help organise code, prevent repetition and make programs easier to understand.

Functions are declared using the `fn` keyword.

## Creating a function

```rust
fn main() {
    greet("Victor");
}

// every parameter in a function needs to have a data type defined
// name is defined as a string slice (&str)

fn greet(name: &str) {
    println!("Hello {}", name);
}
```

Here:
- `fn` declares a function.
- `greet` is the function name.
- `name` is the parameter.
- `&str` defines the parameter's data type.
- `"Victor"` is the argument passed into the function.

## Function parameters and return values

```rust
// a and b are parameters and both are defined as i32
// -> i32 specifies that the function returns an i32 value

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(10, 20));
}
```

The final line does not have a semicolon.

This is an expression, meaning that its resulting value automatically becomes the return value of the function.

Adding a semicolon would turn it into a statement instead.

You can also return a value using the `return` keyword:

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

**Important:**
- Every function parameter must have a specified data type.
- `->` defines the return type of a function.
- A function does not need to return anything.
- The final expression of a function is automatically returned if it does not have a semicolon.

---

# 4. Control Flow

Control flow determines which parts of a program are executed depending on certain conditions.

## 4.1 If and else statements

An `if` statement checks whether a condition is true or false.

Depending on the result, Rust executes the corresponding block of code.

```rust
let number = 6;

// checks whether the remainder of number divided by 2 is equal to 0
// if it is, the number is even
// otherwise, the number is odd

if number % 2 == 0 {
    println!("The number is even!");
} else {
    println!("The number is odd!");
}
```

Here:
- `%` is the remainder operator.
- `==` checks whether two values are equal.
- `if` executes the first block when the condition is true.
- `else` executes the second block when the condition is false.

**Important:**
- Everything inside an if/else statement must be wrapped in curly braces.
- Conditions must evaluate to either `true` or `false`.

## 4.2 Using if statements as expressions

In Rust, an `if` statement can also return a value.

This means you can assign the result of an if/else statement directly to a variable.

```rust
let number = 6;

// checks whether number is even or odd
// if the condition is true, "even" becomes the value of label
// otherwise, "odd" becomes the value of label

let label = if number % 2 == 0 {
    "even"
} else {
    "odd"
};

println!("{}", label);
```

**Important:**
- Both branches must return compatible data types.
- The result can be assigned directly to a variable.
- This works because if/else statements can be expressions in Rust.

---

# 5. Loops

Loops allow you to execute the same block of code repeatedly.

Rust has three main types of loops:

- `loop`
- `while`
- `for`

## 5.1 Loop

A `loop` runs continuously until a `break` command is activated.

```rust
// mutable variable to allow the number to change as the loop runs
let mut counter = 0;

// result is assigned the value returned by the loop
let result = loop {

    counter += 1; // increases counter by one every time the loop runs

    // checks whether counter has reached 5
    if counter == 5 {

        // break terminates the loop and returns counter multiplied by 2
        break counter * 2;
    }
};

println!("Loop result: {}", result);
```

In this example:
- `counter` starts at 0.
- Every iteration increases its value by one.
- When it reaches 5, the if statement becomes true.
- `break` stops the loop.
- The value returned is `5 * 2`, which is 10.

**Important:**
- A `loop` continues indefinitely unless stopped.
- `break` terminates the loop.
- `continue` skips the current iteration and moves to the next one.
- `break` can return a value, which can be assigned to a variable.

## 5.2 While loops

A `while` loop runs as long as its condition remains true.

```rust
// mutable variable starting at 3
let mut n = 3;

// this loop runs while n does not equal 0
while n != 0 {

    println!("{}", n); // prints the current value of n

    n -= 1; // subtracts 1 from n after every iteration
}

// once n reaches 0, the condition becomes false and the loop stops
println!("Liftoff!");
```

The output will be:

```text
3
2
1
Liftoff!
```

**Important:**
- A while loop requires a condition.
- The loop stops when the condition becomes false.
- You need to make sure the condition can eventually become false to avoid an infinite loop.

## 5.3 For loops

A `for` loop is used to iterate over a collection or a range of values.

```rust
// this loop runs through the numbers from 1 up to, but not including, 4

for number in 1..4 {
    println!("{}", number);
}
```

Output:

```text
1
2
3
```

You can also use `..=` to include the final number.

```rust
// this loop includes both 1 and 4

for number in 1..=4 {
    println!("{}", number);
}
```

**Important:**
- `..` excludes the final number.
- `..=` includes the final number.
- For loops are particularly useful when working with arrays and vectors.

---

# 6. Ownership

Ownership is one of the most important concepts in Rust.

It is a system that allows Rust to manage memory safely without needing a garbage collector.

There are three main ownership rules:

1. Each value has an owner.
2. There can only be one owner of a value at a time.
3. When the owner goes out of scope, the value is dropped and its memory is released.

## 6.1 Ownership transfer (move)

```rust
/*
fn main() {
    let s1 = String::from("Hello");

    let s2 = s1;

    println!("{}", s1); // Error!
    println!("{}", s2);
}
*/
```

The reason this causes an error is because ownership of the string has been transferred from `s1` to `s2`.

After this happens:
- `s1` is no longer valid.
- `s2` now owns the string.
- The string can only be accessed through `s2`.

**Why does Rust do this?**

Rust prevents two variables from independently owning the same heap allocation.

If both variables could independently own the same memory, they might both attempt to free it when they go out of scope.

This could cause a double-free error.

Instead, Rust moves ownership from one variable to another.

## 6.2 Cloning

If you want both variables to have their own independent copy of the data, you can use `.clone()`.

```rust
fn main() {
    let s1 = String::from("Hello");

    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

Here:
- `s1` owns the original string.
- `s2` owns a separate copy of the string.
- Changing one string will not affect the other.

Example:

```rust
fn main() {
    let s1 = String::from("Hello");

    let mut s2 = s1.clone();

    s2.push_str(" World");

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
```

Output:

```text
s1 = Hello
s2 = Hello World
```

**Important:**
- `.clone()` creates a separate copy of the data.
- Cloning can require additional memory allocation.
- Both variables can now be used independently.

## 6.3 Stack and heap memory

Understanding the difference between stack and heap memory helps explain why ownership works the way it does.

**Stack:**
- Stores values with a known, fixed size.
- Memory is managed in a last-in, first-out order.
- Commonly used for simple values such as integers and Booleans.

**Heap:**
- Stores dynamically allocated data.
- Memory is allocated when needed.
- Used by types such as `String` and `Vec`.

```rust
let x = 5; // integer value

let s = String::from("Hello"); // string data allocated on the heap
```

A String stores information about its heap allocation, including its pointer, length and capacity.

This is why moving a String has different implications from copying a simple integer.

**Additional note:**

Some types, such as integers, implement the `Copy` trait.

This means that assigning them to another variable copies their value rather than moving ownership.

```rust
let x = 5;
let y = x;

// both variables can still be used because i32 implements Copy
println!("x: {}, y: {}", x, y);
```

---

# 7. Borrowing and References

Borrowing allows you to access a value without taking ownership of it.

Instead of transferring ownership to another variable or function, you can pass a reference to the original value.

References are created using the `&` symbol.

## 7.1 Immutable references

```rust
fn main() {
    let s1 = String::from("hello");

    // &s1 passes a reference to the original string
    // this means calculate_length can access the string without taking ownership of it
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// the parameter is a reference to a String rather than an owned String
// since the function only borrows the string, s1 is still valid in main
fn calculate_length(s: &String) -> usize {

    // .len() returns the length of the string
    // usize is an unsigned integer type commonly used for sizes and indexing
    s.len()
}
```

Here:
- `&s1` creates a reference to `s1`.
- `&String` specifies that the function accepts a reference to a String.
- The function can access the string without taking ownership of it.
- `s1` can still be used in `main` after the function has finished.

**Important:**
- Borrowing allows you to use a value without transferring ownership.
- Immutable references cannot be used to change the original value.
- You can have multiple immutable references to the same value at the same time.

## 7.2 Mutable references

If you want to change a value through a reference, you need to use a mutable reference.

This requires both the original variable and the reference to be mutable.

```rust
fn main() {
    let mut s = String::from("hello");

    // passes a mutable reference to s into the function
    // this allows the function to change the original string
    add_exclamation(&mut s);

    println!("{}", s);
}

// &mut String allows the function to modify the borrowed string
fn add_exclamation(text: &mut String) {

    // push_str adds text to the end of the existing string
    // since text is a mutable reference, this changes s directly
    text.push_str("!");
}
```

Output:

```text
hello!
```

Here:
- `mut` allows the original String to be changed.
- `&mut s` passes a mutable reference into the function.
- `&mut String` specifies that the function accepts a mutable reference.
- `push_str()` adds text to the existing string.

**Important:**
- A variable must be mutable before you can create a mutable reference to it.
- Mutable references allow you to modify the original value without taking ownership.
- Rust only allows one active mutable reference to a particular value at a time.

## 7.3 Borrowing rules

Rust has some restrictions when it comes to borrowing.

These rules prevent multiple parts of a program from making conflicting changes to the same data.

### Multiple immutable references

You can have multiple immutable references to the same variable at the same time.

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}
```

This works because neither reference is allowed to modify the original string.

### Multiple mutable references

You cannot have multiple active mutable references to the same value at the same time.

```rust
/*
fn main() {
    let mut s = String::from("hello");

    // both references are trying to borrow s mutably
    // Rust does not allow multiple active mutable references at the same time

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
*/
```

This causes an error because both references could potentially try to change the same data.

### Mixing immutable and mutable references

You also cannot have an active immutable reference alongside a mutable reference to the same value.

```rust
/*
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // immutable reference
    let r2 = &mut s;  // mutable reference

    println!("{}, {}", r1, r2);
}
*/
```

The problem is that `r1` expects the value to remain unchanged, while `r2` is allowed to modify it.

**Important:**
- You can have multiple immutable references.
- You can have one mutable reference.
- You cannot have multiple active mutable references to the same value.
- You cannot have an active immutable reference alongside a mutable reference to the same value.

Rust checks when references are actually used, rather than simply looking at where they were originally declared. This is part of its borrow-checking system.

---

# 8. Structs

Structs allow you to group related pieces of data together under one custom data type.

Instead of having several unrelated variables, you can combine them into one structure with named fields.

## 8.1 Creating a struct

```rust
// derive Debug is an attribute that automatically generates
// the ability to print the struct using debug formatting

#[derive(Debug)]

// struct User groups related information together under one type
struct User {
    username: String,
    age: u32,
    active: bool,
}

fn main() {

    // creating an instance of the User struct
    // each field must be assigned a value matching its data type

    let user1 = User {
        username: String::from("Victor"),
        age: 18,
        active: true,
    };

    // {:?} is the debug formatting placeholder
    // this is needed because structs do not automatically support normal printing

    println!("{:?}", user1);

    // another way of accessing information inside a struct
    // dot syntax allows you to access individual fields

    println!(
        "{} is {} years old and is {}",
        user1.username,
        user1.age,
        user1.active
    );
}
```

Here:
- `struct` declares a new structure.
- `User` is the name of the struct.
- `username`, `age` and `active` are its fields.
- `User { ... }` creates an instance of the struct.
- Dot syntax accesses individual fields.

**Important:**
- Structs allow you to group different data types together.
- Each field has its own name and data type.
- You need to provide the required fields when creating an instance.

## 8.2 Debug formatting

Rust does not automatically know how you want a struct to be printed.

You can use `#[derive(Debug)]` to automatically generate debug formatting support.

```rust
#[derive(Debug)]

struct User {
    username: String,
    age: u32,
}

fn main() {
    let user1 = User {
        username: String::from("Victor"),
        age: 18,
    };

    println!("{:?}", user1);
}
```

You can also use `{:#?}` to print the struct in a more readable format.

```rust
println!("{:#?}", user1);
```

## 8.3 Methods and impl

You can attach methods to a struct using the `impl` keyword.

Methods are similar to functions, but they are associated with a particular type.

```rust
#[derive(Debug)]

struct User {
    username: String,
    age: u32,
    active: bool,
}

impl User {

    // describe is a method attached to the User struct
    // &self automatically takes a reference to the instance calling the method

    fn describe(&self) -> String {

        // self.username and self.active access the fields
        // belonging to the particular User instance

        format!("{} (active: {})", self.username, self.active)
    }
}

fn main() {

    let user1 = User {
        username: String::from("Victor"),
        age: 18,
        active: true,
    };

    // calling the describe method using dot syntax
    // user1 is automatically passed as the reference to self

    println!("{}", user1.describe());
}
```

Here:
- `impl` is used to implement methods for a struct.
- `&self` is shorthand for a reference to the instance.
- `self.username` accesses the username field of that instance.
- `format!` creates a formatted String rather than printing directly.

**Important:**
- Methods are called using dot syntax.
- `&self` borrows the instance without taking ownership.
- `&mut self` allows a method to modify the instance.
- `self` takes ownership of the instance.

---

# 9. Enums and Pattern Matching

Enums allow you to define a type that can have several possible variants.

Unlike structs, which group multiple fields together, enums represent a value that can be one of several defined possibilities.

## 9.1 Creating an enum

```rust
// enum TrafficLight can only be one of these three variants at a time

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

Here:
- `enum` declares an enumeration.
- `TrafficLight` is the name of the enum.
- `Red`, `Yellow` and `Green` are its variants.

You can create a value using the enum name followed by the variant.

```rust
let light = TrafficLight::Red;
```

## 9.2 Enums with data

Enums can also contain data inside their variants.

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Trapezoid(f64, f64, f64),
}
```

Here:
- `Circle` contains one `f64` value representing its radius.
- `Rectangle` contains two `f64` values representing its width and height.
- `Trapezoid` contains three `f64` values.

This allows different variants of the same enum to hold different amounts of data.

## 9.3 Match expressions

The `match` expression compares a value against different patterns and executes the code belonging to the matching pattern.

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Trapezoid(f64, f64, f64),
}

// Shape is accepted as the parameter so the function can work with
// different variants of the same enum

fn area(shape: Shape) -> f64 {

    match shape {

        // if the shape is Circle, radius receives its stored f64 value
        // the circle area is then calculated using pi * radius squared

        Shape::Circle(radius) => 3.14 * radius * radius,

        // if the shape is Rectangle, width and height receive their values
        // the area is calculated by multiplying them together

        Shape::Rectangle(width, height) => width * height,

        // underscore represents any remaining cases
        // in this example, it catches Trapezoid and returns 0.0

        _ => 0.0,
    }
}

fn main() {

    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(3.0, 4.0);
    let t = Shape::Trapezoid(1.0, 2.0, 3.0);

    println!("Circle area was {}", area(c));
    println!("Rectangle area was {}", area(r));
    println!("Trapezoid area was {}", area(t));
}
```

**Important:**
- `match` compares a value against different patterns.
- Each matching pattern executes its corresponding expression.
- Values inside enum variants can be extracted and used.
- `_` acts as a catch-all pattern for anything not already matched.
- Rust requires match expressions to account for every possible case.

## 9.4 Matching enum variants

You can also use match to check which variant an enum currently contains.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {

    let light = TrafficLight::Red;

    match light {

        // since light is Red, this is the matching condition
        // and "Stop" will be printed

        TrafficLight::Red => println!("Stop"),

        // these conditions will not run because light is not Yellow or Green

        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}
```

**Additional note:**

The underscore `_` can be used when you want to handle any remaining possibilities without specifying each one individually.

However, it is often better to explicitly list every variant when possible, as this makes your code easier to understand and maintain.

# 10. Option, Result and Error Handling

Rust uses special enum types to handle situations where a value might not exist or an operation might fail.

Two particularly important types are:

- `Option`
- `Result`

These allow you to handle potential problems without immediately crashing or stopping the program.

## 10.1 Option

`Option` is used when a value might exist, but it might also be missing.

It has two possible variants:

- `Some(T)` - contains a value.
- `None` - contains no value.

For example, imagine searching for a user using their ID.

```rust
// find_user returns an Option<String>
// meaning it can either return a String or nothing at all

fn find_user(id: u32) -> Option<String> {

    // checks whether the supplied ID is equal to 1

    if id == 1 {

        // if the ID is 1, return Some containing the username

        Some(String::from("Victor"))

    } else {

        // if the ID does not match, return None

        None
    }
}

fn main() {

    let result = find_user(1);

    // match checks which Option variant was returned

    match result {

        // if Some is returned, name receives the String inside it
        Some(name) => println!("Found {}", name),

        // if None is returned, no user was found
        None => println!("No user found"),
    }
}
```

Here:
- `Option<String>` means the function can return either a String or no value.
- `Some()` wraps a value inside the Option.
- `None` represents the absence of a value.
- `match` allows us to handle both possibilities.

**Important:**
- Option is useful when a value might not exist.
- It prevents you from assuming that a value is always available.
- Rust requires you to handle the possible variants when using an exhaustive match.

## 10.2 Result

`Result` is used when an operation can either succeed or fail.

Unlike Option, Result can also provide information about why something failed.

It has two possible variants:

- `Ok(T)` - represents a successful operation and contains the returned value.
- `Err(E)` - represents a failed operation and contains information about the error.

For example, a division function might fail if the divisor is zero.

```rust
// Result<f64, String> means:
// - success returns an f64
// - failure returns a String containing the error message

fn divide(a: f64, b: f64) -> Result<f64, String> {

    // checks whether the divisor is zero
    // dividing by zero is not allowed in this example

    if b == 0.0 {

        // Err indicates that the operation failed
        // String::from creates the error message

        Err(String::from("cannot divide by zero"))

    } else {

        // Ok indicates that the operation was successful
        // the division result is stored inside Ok

        Ok(a / b)
    }
}

fn main() {

    // calls divide and handles both possible outcomes using match

    match divide(10.0, 2.0) {

        // if Ok is returned, value receives the result
        Ok(value) => println!("The result is: {}", value),

        // if Err is returned, error receives the error message
        Err(error) => println!("Error: {}", error),
    }

    // testing another division, this time using negative zero
    // in floating-point arithmetic, -0.0 == 0.0 evaluates to true

    match divide(10.0, -0.0) {

        Ok(value) => println!("The result is: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

**Important:**
- `Result<T, E>` represents either success or failure.
- `Ok()` contains the successful result.
- `Err()` contains information about the failure.
- Both possibilities should be handled.
- Result is particularly useful when working with operations that can fail, such as reading files or parsing input.

## 10.3 The difference between Option and Result

Although both types are used to handle situations where something might go wrong, they serve slightly different purposes.

| Type | Purpose | Possible variants |
|---|---|---|
| `Option<T>` | A value might be missing | `Some`, `None` |
| `Result<T, E>` | An operation might fail | `Ok`, `Err` |

An easy way to remember the difference:

- **Option:** Is there a value?
- **Result:** Did the operation succeed?

## 10.4 The question mark operator (?)

Rust also provides the `?` operator as a shorter way of handling errors.

Instead of manually using match every time, you can use `?` to return an error from the current function if an operation fails.

```rust
fn get_number() -> Result<i32, String> {

    // if parsing succeeds, the number is returned
    // if parsing fails, the error is returned immediately

    let number = "42".parse::<i32>()
        .map_err(|_| String::from("Could not parse number"))?;

    Ok(number)
}
```

Here:
- `?` checks whether the Result is successful.
- If it is `Ok`, the value is extracted and the function continues.
- If it is `Err`, the error is returned from the current function.
- The function must have a compatible return type to pass the error back.

---

# 11. Collections

## 11.1 Vec/vectors

this is a growable array, in contrast to the arrays looked at previously

which was a fixed array where we defined how many numbers the array can contain

in this case we have a vector - Vec<i32> is creating a new vector with a growable list

```rust
fn main() {

    let mut scores: Vec<i32> = Vec::new();
    // every time .push is used on scores, it will add to the end
    scores.push(90);
    scores.push(85);

    for score in &scores {
        println!("score is: {}", score); // prints scores in vector
    }

    println!("first score: {}", scores[0]); // prints specifically the first score
}
```

instead of taking the entire scores variable which would change the ownership to the for loop we use a reference

this will in turn allow for when we loop with scores, it will only be borrowing the vector to read instead of taking ownership

this allows scores to still be used afterward in println!

**important** 

if you only want to read a variable and do something with it ( as here in the for loop) just borrow the value rather than taking ownership of it

**Short explanation of strings**

String is an owned, mutable, heap-allocated string type - use String when you need ownership and mutability

&str is a borrowed, immutable string slice that references a String or a string literal - &str is primarily used for read-only view of a string without ownership

simple example of the difference between &str and String in code
```rust
fn main() {
   
    let owned: String = String::from("hello");
    let borrowed: &str = "world";

    println!("{}, {}", owned, borrowed);
}
```
Output:
```
Hello, world
```

## 11.2 HashMap

A HashMap in Rust is a collection that stores key-value pairs, allowing you to retrieve values using their corresponding keys.

Example code
```rust
use std::collections::HashMap; // Standard library import

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("Victor"), 18);
    ages.insert(String::from("academy"), 49);

    if let Some(age) = ages.get("Victor") {
        println!("Victor is {}", age)
    }
}
```
here:

we use std::collections::HashMap; here, this in turn pulls HashMap in from the standard rust library

.insert is used to insert a key value pair

in the if statement an option is returned, this is because in HashMap, keys inside may not exist

which is why when handling it with if let, .get will return an option.

---

## Iterating through HashMap

In Rust, iterating over a HashMap allows you to access each key-value pair without taking ownership of the map. 

This is typically done using a for loop with references. (Basic iteration shown below)

```rust
    for (name, age) in &ages {
        println!("{} is {}", name, age)
    }
```

Example code
```rust
use std::collections::HashMap; // Standard library import

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("Victor"), 18);

    for (name, age) in &ages {
        println!("{} is {}", name, age)
    }
}
```

**Important:**

Another note is that in HashMaps, iteration order is not guaranteed. you will iterate through every single element, but the order is not guaranteed like in an array.

# 12. Extra Notes

## 12.1 Macros

The exclamation mark at the end of commands such as `println!` indicates that they are macros rather than ordinary functions.

A macro is a way of writing code that generates other code.

Macros allow you to avoid repetition and create flexible, reusable patterns.

For example:

```rust
println!("Hello, world!");
```

The `println!` macro expands into code that formats the text and writes it to the terminal.

A normal function can also print text, but the macro has the advantage of being able to process the special formatting syntax inside the parentheses.

Other examples of commonly used macros include:

- `print!` - prints text without automatically adding a newline.
- `println!` - prints text and adds a newline.
- `format!` - creates a formatted String without printing it.
- `vec!` - creates a vector.
- `dbg!` - prints debugging information about an expression.

**Important:**

The exclamation mark does not always indicate a macro.

For example, `!=` is an operator meaning "not equal to".

```rust
if number != 0 {
    println!("Number is not zero");
}
```

## 12.2 Statements and expressions

One important distinction in Rust is the difference between statements and expressions.

- **Statements** perform an action but do not return a value.
- **Expressions** evaluate to a value.

For example:

```rust
let x = 5; // statement

let y = {
    let a = 3;
    a + 1
}; // expression returning 4

println!("{}", y);
```

The final line inside the curly braces does not have a semicolon.

This means it is an expression, and its value becomes the result of the block.

Adding a semicolon would turn it into a statement.

## 12.3 Scope

Scope refers to the region of code where a variable is valid and can be accessed.

In Rust, variables are generally only available inside the block where they are declared.

```rust
fn main() {

    let x = 5;

    {
        let y = 10;

        println!("x: {}", x);
        println!("y: {}", y);
    }

    println!("x: {}", x);

    // trying to access y here would cause an error
    // this is because y was declared inside the inner scope
}
```

**Important:**
- Variables are valid within their scope.
- When a variable goes out of scope, it is no longer accessible.
- For owned values, Rust also runs the appropriate cleanup when they go out of scope.

---
