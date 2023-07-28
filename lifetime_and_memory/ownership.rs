fn main() {
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // Must use clone instead of plain v2 = v
    println!("{:?}", v);
}