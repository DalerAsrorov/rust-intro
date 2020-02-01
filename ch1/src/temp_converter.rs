use std::io;

struct Scales {
    from: String,
    to: String,
}

fn get_str_value_from_user(choice: i32) -> String {
    let mut value = String::new();
    let str_for_value = match choice {
        1 => "C",
        2 => "F",
        _ => "",
    };

    println!("Type in value (in {}): ", str_for_value);

    io::stdin().read_line(& mut value)
        .expect("Couldn't readl the value");

    value.trim().to_string()
}

fn convert_input_value(value: &String, choice: i32) -> f64 {
    let value = match value.parse::<f64>() {
        Ok(num) => num,
        Err(_err) => 0.0,
    };

    match choice {
        1 => (value * 9.0 / 5.0) + 32.0,
        2 => (value - 32.0) * 5.0 / 9.0,
        _ => 0.0,
    }
}

fn match_scale_opposite_symbol(choice: i32) -> Scales {
    let default_scales = Scales {
        from: "C".to_string(),
        to: "F".to_string()
    };

    match choice {
        1 => default_scales,
        2 => Scales {
            from: "F".to_string(),
            to: "C".to_string()
        },
        _ => default_scales,
    }
}

pub fn start_program() {
  convert_temp_f_and_c();
}

fn convert_temp_f_and_c() {
    loop {
        let mut user_choice = String::new();

        println!("Choose one of the following:");
        println!("\t1. Convert from C to F.");
        println!("\t2. Convert from F to C.");
        println!("Type 0 to quit the program.");

        io::stdin().read_line(& mut user_choice)
            .expect("Failed to read user's choice.");

        let user_choice: i32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(__) => continue
        };

        if user_choice == 0 {
            break;
        } else if user_choice == 1 || user_choice == 2 {
            let value_str = get_str_value_from_user(user_choice);
            let scales = match_scale_opposite_symbol(user_choice);
            let converted_value = convert_input_value(&value_str, user_choice);

            println!("{}({}) is {}({})", value_str, scales.from, converted_value, scales.to);
        }
    }
}
