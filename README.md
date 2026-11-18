Writing down what I've learnt so far here
-

Intro to Variables:
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
    println!("y is {}", x);
    
    let y = y + 1; // new variable
    let y = y * 2; // new variable that uses value of previous variable
    println!("y is {}", y);

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