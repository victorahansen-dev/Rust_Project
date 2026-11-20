Writing down what I've learnt so far here
-

Variables:
-
-------------------

    const MAX_POINTS: u32 = 100_000; //const is always immutable - type is required (example - u32)
    
    // mutable variable can be changed after being set
    // if mut is not put in an error will appear if trying to change it
    let mut x = 5;
    println!("x is {}", x); // curly braces are placeholders
    
    // x is printed twice - first as 5 and then 6
    
    x = 6;
    println!("x is {}", x);
    
    //shadowing - allows to change variable step by step without needing mut
    // each variable is its own and will only be printed once
    let y = 5;
    println!("y is {}", x); // prints "y is 5"
    
    let y = y + 1; // new variable
    let y = y * 2; // new variable that uses value of previous variable
    println!("y is {}", y); // since y became 6 in the second to last variable
    // the last one will now be 6 * 2 and will in turn print out "y is 12"

-------------------

Data types:
-

Scalars
-

    Scalars are single-value data types that include integers, floating-point numbers, Booleans, and characters.
    
    // i32 (signed) is the default for numbers if you don't specify the data type - i32 can have negative numbers
    // u32 (unsigned) is a non-negative 32-bit integer and can't have negative numbers
    // f64 is a 64-bit floating point number for decimals
    // booleans are for assigning a variable to be true or false
    // char is used to print out one single character - adding more will cause an error
    
    // examples of scalars in use
    let age : u32 = 18;
    let num : i32 = -100; 
    let temperature : f64 = 36.6;
    let is_active: bool = true;
    let grade: char = 'A';
        
    println!("age: {}, number: {} temp: {}, active: {}, grade: {}",
        age, num, temperature, is_active, grade
    ); //prints all variables in a list

Compound types - lets you mix different data types
-

    // tuple - groups values of different types together accessed with variable name .0, .1 etc.
    // in tuples you denote them using small brackets where you write the type for every single tuple being added and assign it corresponding to that type
        
    // let person under as example of this - string should be a string and an unsinged 32 should be an unsigned 32 etc.
    let person : (&str, u32, u32) = ("Maya", 29, 50000);
    println!("{} is {} years old, has a salary of {}", person.0, person.1, person.2);
        
    // array - making a fixed size of numbers/types/values
        
    // all values will be assigned the same type (singed 32-bit values in this scenario as seen below)
    // which are a fixed size of 3 numbers and can be accessed using 0,1,2 and so on
    let scores : [i32; 3] = [90, 85, 100];
    println!("first score is: {}", scores[0])

-------------------

Functions:
-

    // function result is printed out inside of main
    fn main() {
    greet("Victor");
    println!("{}", add(10, 20));
    }
    
    // every paramater in a function needs to have a data type defined
    // example of this is seen in greet where name is defined as a string
    fn greet(name: &str) {
    println!("Hello {}", name);
    }
    
    // another example of paramaters where a and b are defined as i32
    fn add(a: i32, b: i32) -> i32 {
    a + b // this is an expression - because of its lack of a semicolon
    // its value automatically becomes the return value of the function
    // putting a semicolon in front of the expression will make it a statement and cause an error
    }

-------------------
Control flow:
-
if/else statements
-

    let number = 6; // our number variable
    
    // everything inside an if/else statement must be wrapped in curly braces
    // this if statement checks how many times 2 can be fit into the number variable (6 in this case)
    // afterward if the result is equal to 0, it will print the text written below in the if statement
    // if it's not, it will print out the text in the else statement
    
    if number % 2 == 0 {
            println!("The number is even!")
        } else {
            println!("The number is odd!")
        }

Using if statements as expressions
-

    let number = 6;

    // same logic here with number variable as previously
    // returns even if number is two and the remainder is 0 which then turns label's value into even
    // otherwise odd will be returned and become the value of label
    let label = if number % 2 == 0 {"even"} else {"odd"};

    println!("{}", label)

-------------------

Loops:
-

    // a loop runs infinitely until the terminal is shut off or when a break command is activated
    // mutable variable to allow the number to change as the loop runs
    let mut counter = 0;
    let result = loop { // result is defined into a loop here
        counter += 1; // counter variable increases by one every time the loop runs
        if counter == 5 { // if the counter variable is 5
            break counter * 2 // the break command will run which terminates the infinitely running loop
            // after the loop ends, the number the counter returns is multiplied with 2
    
        }
    };

    println!("loop result: {}", result); // in this case the result is 10 because the equation is 5 * 2
    // while loop
    let mut n = 3;
    while n!= 0 { // this loop runs while n does not equal 0
        println!("{}", n); // prints current value of n
        n-=1; // subtracts current value of n by 1
    }; // this loop will run continuously until the value of n has reached 0
    // since the starting value is 3, it will count down from 3
    println!("Liftoff!"); // when n has reached 0, this text will print

-------------------

Ownership:
-

    // this is just an example of how ownership transferring errors
    /*fn main() {
        let s1 = String::from("Hello"); // creates a heap allocated growable string different from &str
        let s2 = s1; // the ownership of the heap data has been transferred to s2 from s1
    
        println!("{}", s1); // which causes an error here
        println!("{}", s2); // because s2 now has ownership of the heap data
        // the point of this is to prevent bugs where both s1 and s2 in which
        // they both think they have access to the "::from("Hello");" memory
        // but s2 can only access the memory making s1 redundant
    } */

Cloning
-

    /*fn main() {
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // instead of the data being transferred to s2
        //both variables has a heap allocated growing string, but is different from the one s1 has
        // changing s2 will have no effect over s1 which allows both to be used independently
    
        println!("s1 = {}, s2 = {}", s1, s2); // since the heap data of s2 was cloned they will both print the same text in the string
        // changing the one in s2 will not have an effect on s1

    }*/

-------------------

Borrowing and references:
-

    fn main() {
        let s1 = String::from("hello"); // This is an owned string created in the main function
        let len = calculate_length(&s1); // s1 is passed as a reference into the calculate_length function instead of the value
        of s1 itself
        
        println!("The length of '{}' is {}.", s1, len) // this prints out the result - s1 is the string itself and len is the length of the string
    }
    
    
    // inside the function the parameter is of the type &string - which is a borrowed string
    // since the function is only borrowing the string, s1 is still valid in the main function
    // if the function were to also own the string, this function would now be the viable owner of the s1 variable, and
    would not allow you to access it in main anymore
    // this would in turn cause an error in println!, because main is still trying to use s1 after ownership has been moved
    fn calculate_length(s: &String) -> usize {
    s.len() // inside the function we return the length of the string
    }

    /*fn main() {
        let mut s = String::from("hello");
        // while this looks like it may work because the string is mutable
        // there will be an error - this is because both r1 and r2 are trying to borrow the s variable
        // the issue is that mutable variables can only be borrowed once
        let r1 = &mut s;
        let r2 = &mut s;
    
        println!("'{}', '{}'", r1, r2);
    } */
    
    fn main() {
        let mut s = String::from("hello");
        add_exclamation(&mut s); // this passes a single mutable borrow into a function that needs to change the value
        println!("{}", s)
    }
    
    fn add_exclamation(text: &mut String) { // through this reference, the function can change the value of the s string without taking ownership nor moving the string itself in any way
    text.push_str("!") // push str mutates the string in place through the reference
    }

-------------------
Structs:
-

    // derive Debug is an attribute asking Rust to auto generate the ability to print the struct for debugging

    #[derive(Debug)] // auto generates debug printing support
    
    // struct User is a struct group - a struct group relates grouped data together under one type with named fields
       
    // everything here has different data, but the user itself has only one type (user itself)
    
        struct User {
            username: String,
            age: u32,
            active: bool,
        }
        
        fn main() {
            let user1 = User {
            username: String::from("Victor"),
            age: 18,
            active: true,
        };
        
            /*println!("{:?}", user1,); // the colon question mark inside the curly braces is the debug format placeholder
            // this is needed because struct does not print with normal curly braces */
        
            // another way of printing out the text in the struct is by using dot syntax to access the individual fields
            println!("{} is {} years old and is {}", user1.username, user1.age, user1.active);
        
            // this is the print command for impl
            println!("{}", user1.describe()) // since describe is a method attached to user1 using impl
            // the describe method automatically takes in its reference and is going to format the string, in turn this formatted string is going to be replaced at the placeholder
            // user1.describe now has both self.username and self.active which are user1.username and user1.active
        }
        
        // you can also attach a method to a struct using impl
        // if user1 uses this describe method, a reference of itself is going to be passed as an argument
        // so the struct itself is going to be passed as an argument and is accessed using self.username
        // like how user1.username was accessed, but it does it automatically for you
        
        impl User {
            fn describe(&self) -> String { // the describe function takes in a reference of self and returns a string
            format!("{} (active : {})", self.username, self.active) // here the string is formatted
            }
        }

-------------------

Enums and pattern matching:
-

    enum TrafficLight { // this is an enum, it can hold only one of these variants at a time
    Red, // variants without any data
    Yellow,
    Green,
    }
    
    enum Shape {
    Circle(f64), // variants with data
    Rectangle(f64, f64),
    Trapezoid(f64, f64, f64)
    }
    
    fn area(shape: Shape) -> f64 { // putting Shape in the parameters allows for both Circle and Rectangle to be accessed in one function
    match shape { // checks if the enum (Shape) matches with the conditions (Circle and Rectangle)
    // in this case both do, which means both will be calculated
    Shape::Circle(radius) => 3.14 * radius * radius,
    Shape::Rectangle(width, height) => width * height,
    _ => 0.0, // since the trapezoid is not accounted for as a condition, this takes any other case aside from the two already inside the function and returns 0.0
    }
    }
    
    fn main() {
    let light = TrafficLight::Red;
    
        let c = Shape::Circle(2.0); // because Circle has a float64 data type, you must add a number corresponding based on what data type used
        let r = Shape::Rectangle(3.0, 4.0); // same logic here as Circle
        let t = Shape::Trapezoid(1.0, 2.0, 3.0);
    
        println!("circle area was {}", area(c));
        println!("rectangle area was {}", area(r));
        println!("Trapezoid area was {}", area(t));
    
        match light{ // match compares a value against each possible pattern and runs the matching arm
            TrafficLight::Red => println!("Stop"), // match is going to compare to this condition, since they are both red the condition will be true and print out "Stop"
            TrafficLight::Yellow => println!("Slow down"), // when match checks Yellow, the condition will instead be false, in turn not printing out anything
            TrafficLight::Green => println!("Go"), // same logic here as Yellow in this scenario
        }
    }

-------------------

Option, Result and Error Handling
-

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
            Some(name) => println!("Found {}", name)
        }
    }

-------------------

Extra notes:
-

    // the exclamation point on commands (like println!) indicates it being a macro.
    // a macro is a way to write code that generates other code
    // this lets you avoid repetition and create flexible, reusable patterns.
    
    // The macro expands into code that formats the text and writes it to the terminal. 
    // A normal function can also print text
    // the macro’s advantage is that it can process the special formatting syntax inside the parentheses.
    
    // There are exceptions to this
    // for instance, exclamation points can also be used as "does not equal"
    // this is mainly used in loops, functions and if/else statements

-------------------
