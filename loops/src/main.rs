fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("{}\n", result);

    while_loop();
    println!("\nRun for loop...");
    for_loop();
    println!("\nRun for loop with rev...");
    for_loop_with_rev();
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFT OFF!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}", element);
    }
    println!("LIFT OFF!");
}

fn for_loop_with_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}