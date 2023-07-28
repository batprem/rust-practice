#![allow(dead_code)]
#![allow(unused_variables)]

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

fn method() {
    let p = Point {x: 3.0, y: 4.0};
    let p2 = Point {x: 5.0, y: 10.0};
    let myline = Line { start: p, end: p2};

    println!("length = {}", myline.len())
}

fn main() {
    method();
}