#![allow(unused_mut)]
#![allow(unused_variables)]


use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new (re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;

    // a + b
    fn add (self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output = T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> PartialEq for Complex<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T :Eq> Eq for Complex<T> where T: Eq {}

fn main() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);

    println!("{:?}", a + b);
    a += b; // Add assign
    println!("{:?}", a);
    println!("Minus a {:?}", -a);
    println!("a == a {:?}", a == a);
}