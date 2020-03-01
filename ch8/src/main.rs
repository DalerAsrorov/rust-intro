mod spreadhseet_cell;
mod string_methods;

use std::collections::HashMap;
use spreadhseet_cell::SpreadsheetCell;

fn main() {
    // --- Vectors Overview ---
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}!!!", third),
        None => println!("There is no third element.")
    }

    v.push(6);

    for i in &v {
        println!("element: {}", i);
    }
    // change each element inside of the vector 'v'
    for i in &mut v {
        *i += 50;
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for el in &row {
        println!("element: {:?}", el);
    }

    let last_el = row.pop();

    println!("last element from row: {:?}", last_el);

    // --- Strings Overview ---

    let mut new_s = String::new();
    new_s.push('f');
    new_s.push_str("bar");

    let mut new_s1 = String::from(&new_s);

    new_s1.push_str(&new_s);

    println!("new_s1='{}'", new_s1);

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}, {}", s1, s2);

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    // the format macro doesn't take ownership of its params
    let ss = format!("{}-{}-{}", ss1, ss2, ss3);

    println!("{}\n", ss);

    // also can use ss.bytes() to access each byte
    for c in ss.chars() {
        println!("ss char: {}", c);
    }

    // --- Hash maps Overview ---
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let new_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}\n", new_scores.get(&"Blue".to_string()));

    // a. overwriting a value
    scores.insert(String::from("Blue"), 23);
    // b. only inserting a value if the key has no value - entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Darn")).or_insert(50);
    // c. updating value based on the old value


    // iterating over hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Result: {:?}", string_methods::count_words(&"How are you Daler are you".to_string()))
}
