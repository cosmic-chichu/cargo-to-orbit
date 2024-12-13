use std::time::Duration;
use std::time::Instant;
use time::macros::datetime;

fn main() {
    let now = Instant::now(); 
    let five_seconds = Duration::from_secs(5);

    println!("{:#?}", now);
    let after_five_seconds = now + five_seconds;

    println!("{:#?}", after_five_seconds);

    // struct Foo {
    //     x: i32
    // }

    // let bar = Foo {x: 5};
    // println!("{:?}", bar);

    let start = datetime!(2022-01-02 11:12:13.123_456_789);

    let start_after_five_seconds = start + five_seconds;
    println!("{start}");
    println!("{start_after_five_seconds}");

}
