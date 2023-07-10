
mod shapes;
use crate::shapes::Rectangle;
use crate::shapes::Triangle;
use crate::shapes::Circle;
use crate::shapes::Shape;

use std::io;

fn main() {
    let rectangle = Rectangle::new(5.0, 4.0);
    let triangle = Triangle::new(2.0, 5.0);
    let circle = Circle::new(5.0);
loop {
    println!("Please input r, t or c.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let shape = match &guess.trim()[..] {
        "r" => Shape::Rec(rectangle),
        "c" => Shape::Circ(circle),
        "t" => Shape::Tri(triangle),
        _ => continue,
    };

    let area: f64 = match &shape {
        Shape::Rec(rectangle) => shape.unwrap().area(),
        Shape::Circ(circle) => shape.unwrap().area(),
        Shape::Tri(triangle) => shape.unwrap().area(),
    };

    println!("your shape is {} and has area {}", shape.unwrap().shape_name(), area);
    break;
}
}