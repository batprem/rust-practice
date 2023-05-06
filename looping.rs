#![allow(dead_code)]


pub fn while_loop() {
    let mut x: i8 = 0;
    while x < 10 {
        println!("x: {}", x);
        x += 1;
    }

    x = 0;
    loop {
        if x == 10 { break; } 
        println!("x: {x}");
        x += 1;
    }
}

pub fn for_loop() {
    for (i, x) in (1..11).enumerate() {
        println!("i,x: {i} {x}");
    }
}
fn main(){
    // while_loop();
    for_loop();
}