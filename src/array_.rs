use std::mem;

fn array() {
    // Note: Array will be fixed size
    let mut a: [i32; 5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("{:?}", a);

    if a != [321,2,3,4,5] {
        println!("Does not match");
    } else {
        println!("Match");
    }

    let b = [1; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));
    let mtx:[[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
    ];
    println!("{:?}", mtx);

    // Print diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("mtx[{i}][{j}] = {}", mtx[i][j]);
        }
    }
}

fn use_slice(slice: &mut[i32]) {
    println!("First elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data)
    println!("{:?}", data);
}

fn main() {
    // array();
    slices();
}