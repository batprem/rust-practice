struct Person {
    name: String
}

impl Person {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> { // lifetime z
    name: String,
    ceo: &'z Person
} // Guiding rust that the lifetime of Company and the lifetime of ceo are the same

fn main() {
    let mut z: &String;
    {
        let p = Person { name: String::from("John")};
        z = p.get_ref_name();
        println!("Name: {}", z)
    }
}