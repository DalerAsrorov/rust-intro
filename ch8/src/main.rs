fn main() {
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
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("element: {}", i);
    }
}
