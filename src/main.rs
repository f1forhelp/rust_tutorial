fn main() {
    //-------Immutable variables-------
    let x = 5;

    // we cant mutate the value of x because it is immutable
    // by default in rust variables are immutable

    // x = 32; // error: cannot mutate immutable variable `x`

    println!("The value of x is: {}", x);

    //-------Mutable variables-------
    // to make a variable mutable we need to use the mut keyword
    let mut y = 5;
    y = 32; // this is valid

    println!("The value of y is: {}", y);

    //-------Constants-------
    // constants are always immutable
    // constants are declared using the const keyword
    // constants can be declared in any scope, including the global scope
    // constnats must be annotated with a type
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    //-------Tuple-------
    // tuples can be used to store multiple values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    print!("The value of tup is: {}, {}, {}", tup.0, tup.1, tup.2);

    // we can also destructure a tuple
    let (x, y, z) = tup;

    print!("The value of x, y, z is: {}, {}, {}", x, y, z);

    //-------Array-------
    // arrays in rust have a fixed length
    // arrays in rust are homogenous

    let arr = [1, 2, 3, 4, 5];

    // we can access the elements of an array using the index
    println!("The value of arr[0] is: {}", arr[0]);

    // we can also access the elements of an array using the get method
    // this method returns an Option
    match arr.get(1) {
        Some(val) => println!("The value of arr[1] is: {}", val),
        None => println!("The value of arr[1] is: None"),
    }
}

//-------Functions-------
fn sum(x: u64, y: u64) -> u64 {
    //We can do an implicit return in rust
    // return x + y;
    x + y
}

fn controlFlow() {
    //-------If-------
    let number = 3;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    //-------If else if-------
    let number = 6;

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    //-------If in a let statement-------
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    //-------Loops-------

    //-------Loop-------
    // loop {
    //     println!("again!");
    // }

    //-------While-------
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //-------For-------
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //-------For with range-------
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
