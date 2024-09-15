pub fn main() {
    let x = 5;
}

fn variables() {
    let x = 5;
    println!("Value of x is :{}", x);
}

fn constants() {
    //We cant create a const variable of type String as its not of primitive type.
    // const SCHOOL_NAME: String = String::from("");

    //We need to specify type whenever we try to define Const.
    const SCHOOL_NAME: &str = "KV School";
}

fn scalarTypes() {
    //Integer Type

    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128 u128
    // arch	    isize usize

    //Floating point type

    // 16-bit	f16
    // 32-bit	f32
    // 64-bit	f64
    // 128-bit  f128

    // Character type (charcaters are enclosed in single quotes)
    let gender: char = 'f';

    // Boolean type.
    let isPaid: bool = false;
}

fn compoundTypeTuple() {
    let tup: (i32, f64, char) = (12, 1.0, 'w');

    let (x, y, z) = tup;

    println!("Tuple first value: {}", tup.0);
    println!("Tuple second value: {}", tup.1);
    println!("Tuple third value: {}", tup.2);
}

fn compundTypeArray() {
    let arr = [1, 2, 3];

    //Will generate an array of length 5 having value 3.
    let arr1 = [3; 5];
}

fn functionWithoutReturn() {
    println!("Hello world");
}

fn functionWithReturn(x: i32, y: i32) -> i32 {
    let sum = x + y;
    //We can ommit the return and can directly right sum.
    // return sum;
    sum
}

fn ifElse() {
    let condition = false;

    //We can use if else in assignment as we can directly return valures from function brackets.
    let number = if condition { 5 } else { 4 };
}

fn loops() {
    //We can return values form loop in rust.

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("Counter is :{}", result);

    while counter != 0 {
        counter -= 1;
    }

    let a = [10, 12, 34];

    for value in a.iter() {
        println!("Value is :{}", value);
    }

    for value in 1..4 {
        println!("Value is :{}", value);
    }

    //For getting index we can use enumerate.
    for (index, value) in a.iter().enumerate() {
        println!("Index is {index} and value is {value}")
    }

    loop {
        println!("This will not stop");
    }
}
