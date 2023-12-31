#![allow(unused)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


mod sh;
mod conditions;
mod looping;
mod src;
mod characters_and_strings;

use src::run_matching;
use src::rm;
use src::combination_lock;
use characters_and_strings as cas;


fn max<'a>(x: &'a i32, y: &'a i32) -> i32 {
    if *x > *y { *x } else { *y }
}
fn main() {
    // println!("Hello, world! test");
    // sh::stack_and_heap();
    // conditions::if_statement(30);
    // looping::for_loop();
    // rm(44);
    // combination_lock();
    cas::run_guessing_game();
}