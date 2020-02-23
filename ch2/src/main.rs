mod slice_type;

fn main() {
    let s = String::from("How da hell?");

    let first_word: &str = slice_type::first_word(&s);
    let second_word: &str = slice_type::second_word(&s);

    println!("first_word: {}", first_word);
    println!("second_word: {}", second_word);

    // let slice = &s[3..s.len()];
    // println!("slice 1: {}", slice);
    // let slice = &s[3..];
    // println!("slice 2: {}", slice);

}
