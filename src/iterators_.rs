pub fn main() {
    let mut vec = vec![3, 2 , 1];
    for x in &vec {
        println!("we got {}", *x);
    }

    for x in vec.iter() {
        println!("we got {}", x);
    }

    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    // Reverse
    for x in vec.iter().rev(){
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1, 2 , 3];
    let it = vec2.clone().into_iter();
    vec2.extend(&vec);
    println!("{:?}", vec2);

}