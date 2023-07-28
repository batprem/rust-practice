#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, Arc};


struct Person {
    name: Arc<String>,
    state: Arc<Mutex <String>>
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name: name, state: state }
    }
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
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

fn mutex_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(
        Mutex::new("bored".to_string())
    );
    let person = Person::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}


fn main() {
    mutex_demo();
}