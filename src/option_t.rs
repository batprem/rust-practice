fn main() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v)| None
    let result = if y != 0.0 {Some(x/y)} else { None };
    
    match result {
        Some(z) => {println!("{x}/{y}={z}");},
        None => println!("Cannot divide by zero")
    };

    // Result is None, not set the value to z
    if let Some(z) = Some(x/y){
        println!("result = {z}");
    } else {
        println!("Abort");
    }
}