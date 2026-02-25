use std::{io, mem::size_of_val};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin_point() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin_point() -> Box<Point> {
    // Box::new(origin_point())
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // Stack allocated variables (point and rectangle)
    let point: Point = origin_point();

    let rectangle: Rectangle = Rectangle {
        top_left: point,
        bottom_right: Point { x: 4.0, y: -4.0 },
    };

    // Heap allocated variables
    let boxed_point: Box<Point> = boxed_origin_point();
    // let boxed_point: Box<Point> = Box::new(Point { x: 0.0, y: 0.0 });
    
    // let boxed_rectangle: Box<Rectangle> = Box::new(rectangle);
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin_point(),
        bottom_right: Point { x: 4.0, y: -4.0 },
    });

    println!("Stack Point occupies: {} bytes", size_of_val(&point));
    println!("Stack Rectangle occupies: {} bytes", size_of_val(&rectangle));
    println!("Heap Point occupies: {} bytes", size_of_val(&boxed_point));
    println!("Heap Rectangle occupies: {} bytes", size_of_val(&boxed_rectangle));
    println!("Unboxed Rectangle occupies: {} bytes", size_of_val(&*boxed_rectangle));
}
