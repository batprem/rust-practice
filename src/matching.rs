#![allow(dead_code)]

pub fn run_matching<'matching>(country_code: i32) -> &'matching str{
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("The country with code {country_code} is {country}");
    country
}