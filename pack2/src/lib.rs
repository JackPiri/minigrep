mod front_of_house;

pub use front_of_house::hosting1;

pub fn eat_at_restaurant() {

    front_of_house::front_of_house_f();
    front_of_house::hosting1::add_to_waitlist1();
    front_of_house::hosting1::hosting1_1::add_to_waitlist1_1();
    front_of_house::hosting2::add_to_waitlist2();

    println!("eat_at_restaurant done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_restaurant() {
        let x = 2;
        assert_eq!(x, 2);
    }
}