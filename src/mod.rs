#![allow(dead_code)]
mod matching;
mod combination_lock;
pub use matching::run_matching as rm;
pub use combination_lock::combination_lock;


pub fn run_matching() {
    let country = rm(44);
    println!("Country: {country}")
}