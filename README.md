Writing down what I've learnt so far here
-

Variables:
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
-------------------

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

Functions
-------------------

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

Control flow
-------------------

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

Loops
-------------------

Regular loops
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
    
While loops
-
    let mut n = 3;
    // a while loop runs until the right conditions are met
    while n!= 0 { // in this case it will run until the value of n equals 0
        println!("{}", n); // prints current value of n
        n-=1; // subtracts current value of n by 1
    }; // this loop will run continuously until the value of n has reached 0
    // since the starting value is 3, it will count down from 3
    println!("Liftoff!"); // when n has reached 0, this text will print

For loops
-
    // this is a for loop - this allows you to loop over ranges and arrays
    for i in 1..4 { // this loop prints out all numbers from the starting point (1)
        println!("{}", i); // to the last number right below what has been written (3 in this case)
    }
Arrays in a for loop
-
    let scores = [90, 85, 100]; // here we define an array
    for score in scores { // this loops through every item in the array once
        println!("score is: {}", score);
        // the 3 items in the array work as the value of the score variable changing
        // and every time the value changes the text is printed with the new value/score
    }

-------------------

Extra notes
-------------------

    // the exclamation point on commands (like println!) indicates it being a macro.
    // a macro is a way to write code that generates other code
    // this lets you avoid repetition and create flexible, reusable patterns.
    
    // The macro expands into code that formats the text and writes it to the terminal. 
    // A normal function can also print text
    // the macro’s advantage is that it can process the special formatting syntax inside the parentheses.
    
    // There are exceptions to this
    // for instance, exclamation points can also be used as "does not equal"
    // this is mainly used in loops, functions and if/else statements that contain numbers

-------------------
