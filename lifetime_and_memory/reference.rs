#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::thread;
use std::sync::Arc;


struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

// fn rc_demo(){
//     let name = Rc::new("John".to_string());
//     println!(
//         "Name = {}, name has {} strong poninters",
//         name,
//         Rc::strong_count(&name)
//     );
//     {
//         let person = Person::new(name.clone());
//         println!(
//             "Name = {}, name has {} strong poninters",
//             name,
//             Rc::strong_count(&name)
//         );
//         person.greet();
//     }

//     println!(
//         "Name = {}, name has {} strong poninters",
//         name,
//         Rc::strong_count(&name)
//     );
// }

fn arc_demo() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    t.join().unwrap();
}

fn main() {
    arc_demo();
}