#![allow(dead_code)]
mod matching;
mod combination_lock;
mod enumurate;
mod union_;

pub use matching::run_matching as rm;
pub use combination_lock::combination_lock;
pub use enumurate::demo as enum_demo;
pub use union_::process_value;


pub fn run_matching() {
    let country = rm(44);
    println!("Country: {country}")
}