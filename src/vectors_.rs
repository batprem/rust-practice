fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);

    // usize isize

    let idx:usize = 0; // Index must me usize
    println!("a[0] = {}", a[0]);
    println!("a[{idx}] = {}", a[idx]);

    // Change value
    a[idx] = 312;
    println!("After change value");
    println!("a = {:?}", a);

    // Handle out of index
    let idx2:usize = 6;
    // a.get(idx2); // Prevent program crashing
    match a.get(idx2) {
        Some(x) => println!("a[{idx2}] = {}", x),
        None => println!("error, no such element")
    }

    // Iteration
    for x in &a {
        println!("{}", x);
    }

    // Remove element
    let last_elem_0 = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem_0, a);
    while let Some(last_elem) = a.pop(){
        println!("last elem is {}, a = {:?}", (last_elem), a);
    }
}

fn main() {
    vectors()
}