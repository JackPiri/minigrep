pub mod hosting1;
pub mod hosting2;

pub fn front_of_house_f() {
    let myvar = 8;
    let foo = 10;
    println!("foo {}", foo);
    hosting1::add_to_waitlist1();
    hosting1::hosting1_1::add_to_waitlist1_1();
    hosting2::add_to_waitlist2();
}