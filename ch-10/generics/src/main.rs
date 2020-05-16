#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}", integer.x());
    println!("float distance from origin = {}", float.distance_from_origin());

    let p1 = DualPoint { x: 5, y: 10.4 };
    let p2 = DualPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct DualPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> DualPoint<T, U> {
    fn mixup<V, W>(self, other: DualPoint<V, W>) -> DualPoint<T, W> {
        DualPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
