fn main() {

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
}