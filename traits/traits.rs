trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Sumable<T> {
    fn sum(&self) -> T;
}

impl Sumable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self { result += *x}
        return result;
    }
}

fn traits() {
    let h = Human::create("John");
    h.talk();

    let c = Cat{name: "Misty"};
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

fn print_info(shape: impl Shape) {
    println!("The area is {}", shape.area())
}

fn main() {
    traits();
    let c = Circle { radius: 2.0 };
    print_info(c);
}