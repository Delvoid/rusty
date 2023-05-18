mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            todo!()
        }

        pub fn seat_at_table() {
            todo!()
        }
    }

    mod serving {
        fn take_order() {
            todo!()
        }

        fn serve_order() {
            todo!()
        }

        fn take_payment() {
            todo!()
        }
    }
}

fn deliver_order() {
    todo!()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {
        todo!()
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    // previous use not in scope
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
