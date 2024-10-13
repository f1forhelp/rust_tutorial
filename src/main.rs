fn main() {
    //Ownership and Borrowing
    // memories and heap tutorial [https://www.youtube.com/watch?v=_8-ht2AKyH4]

    //------Ownership rules in Rust------
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    //------Ownership and Functions------
    //When we pass a variable to a function, the ownership of the variable is moved to the function.
    //This is called move in Rust.
    let str1 = String::from("Hello");

    //In below function we are moving the ownership of str1 to takesOwnership function assignment[paramenter]
    takes_ownership(str1);

    //Below print statement will give error as str1 is moved to takesOwnership function
    //Err:
    // borrow of moved value: `str1`
    // value borrowed here after move
    //
    // println!("{}", str1);
}

fn copy_example() {
    let x = 5;
    let y = x;

    //Both x and y are pointing to two diffrent location in memory[stack] they are copied instead of moved.
    //So we can access both the values as seen below.
    //In rust all the primitive types are stored in stack and are copied instead of move.
    println!("x = {}, y = {}", x, y);
}

fn move_example() {
    let str1 = String::from("Hello");
    let str2 = str1;

    //Below print statement will give error as str1 is moved to str2
    //Err:
    // borrow of moved value: `str1`
    // value borrowed here after move
    //
    // println!("str1 = {} , str2 = {}", str1, str2);

    //To fix above error we can use clone method to create a deep copy of str1
    //Now they will be two diffrent pointer to two diffrent location in memory[heap]
    //Note: String types are smartPointer so ther are stored in heap.
    let str1 = String::from("Hello");
    let str2 = str1.clone();
    println!("str1 = {} , str2 = {}", str1, str2);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn pass_by_refrence_get_str_length(str: &String) -> usize {
    //Passing value by refrences will not move the value and hence usulally in rust .
    //We do pass by refrecne in function call for below two reason.
    //1. We avoid copying data
    //2. As we are only borrowing not moving the value [passing refrence are also called borrowing.].
    return str.len();
}
