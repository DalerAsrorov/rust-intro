use std::io;

pub fn start_program() {

  loop {
    let mut user_input = String::new();

    println!("Please enter a value or press Q to quit:");

    io::stdin().read_line(& mut user_input)
      .expect("error reading value");

    let user_input = user_input.trim();

    match user_input {
      "q" => break,
      "Q" => break,
      _ => {
        let n = match user_input.trim().parse::<i32>() {
          Ok(num) => num,
          Err(__) => continue,
        };

        if n < 0 {
          println!("Negative values are not accepted.");
          continue;
        }

        println!("Fib num for n = {} is {}", n, fib(n));
      }
    }
  }
}

fn fib(n: i32) -> i32 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib(n - 1) + fib(n - 2)
  }
}