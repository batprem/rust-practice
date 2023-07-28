pub union IntOrFloat{
    i: i32,
    f: f32
}

pub fn process_value(iof: IntOrFloat) -> f32 {
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("meaning of life value");
                iof.i as f32
            }
            IntOrFloat {f} => {
                println!("value = {f}");
                f
            }
        }
    }
}
fn main() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // let value = unsafe {
    //     iof.i
    // };
    let value = unsafe{iof.i};
    println!("iof.i {value}");
    process_value(IntOrFloat{i: 42});
    let new_value = process_value(IntOrFloat{i: 42});
    println!("New value {}", new_value)
}