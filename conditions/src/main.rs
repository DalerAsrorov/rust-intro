fn main() {
    let number = 101;

    if number == 0 {
        println!("is equal to 0");
    } else if number < 100 {
        println!("is less than 100");
    } else {
        println!("is more than 100");
    }

    let isEven = false;

    let number = if isEven {
        4
    } else {
        5
    };

    println!("Num: {}", number);
}
