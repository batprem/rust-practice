fn strings() {
    // utf-8
    let s:&'static str = "hello there!"; // &str = string slice

    for c in s.chars(){
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // String (heap)
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters); 

    // &str <> String
    let u:&str = &letters;
    // concatentation
    // String + str
    let z = letters.clone() + &letters;
    println!("{:?}", z);

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn main() {
    strings();
}