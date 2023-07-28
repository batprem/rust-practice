fn main() {
    let print_vector = |x: &Vec<i32>|  {
        // Borrow vector
        println!("{:?}", x);
    };
    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    /*
    let b = &mut a;
    *b += 2;
    println!("a = {}", a); // Error becuase value is borrowed
    */
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a); // Not error

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!{"i = {}", i};
        // z.push(5) // error
    }

}