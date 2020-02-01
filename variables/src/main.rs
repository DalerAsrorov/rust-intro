fn main() {
    // mutable vars
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 100_000_000;

    println!("Max points: {}", MAX_POINTS);

    // tuples
    let tup0: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of (x, y, z) is: ({}, {}, {})", x, y, z);
    println!("Try print tuple {:?}", tup0);
    println!("Another try {}", tup0.0);

    // arrays
    // let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let arr1 = [3; 5];

    println!("Array {:?}", arr1);

    let result_num = add_two_nums(100, 50);

    println!("Result {}", result_num);
}

fn add_two_nums(num1: i32, num2: i32) -> i32 {
    num1 + num2
}