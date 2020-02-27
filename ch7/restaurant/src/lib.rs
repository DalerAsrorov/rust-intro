use std:: {io, cmp::Ordering};

mod front_of_house;

mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

use crate::front_of_house::hosting;
// Relative path version of 'use' below:
// use self::front_of_house::hosting;

pub use back_of_house::Breakfast as BaseBreakfast;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::serving::serve_food();

    // using the namespace from 'use' keyword
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_two() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = BaseBreakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    // The code below is not allowed since seasonal_fruit is private
    // println!("Fruit choice: {}", meal.seasonal_fruit);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("{:?}, {:?}", order1, order2);
}