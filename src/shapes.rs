//Enum shape has three versions rectangle, triangle and circle
#[derive(Debug)]
pub enum Shape {
    Rec(Rectangle),
    Tri(Triangle),
    Circ(Circle),
}

// What is common to all shapes?? Every shape has area and name. That is the Trait we are impementing for all shapes
pub trait ShapeTrait {
    // Common method that all shapes have in comon
    fn area(&self) -> f64;
    fn shape_name(&self) -> String;
}

// unwrap method will take Shape object (self) and will return the object that implements ShapeTrait (rectangle, circle or triangle) - "unwraping it"
impl Shape {
    pub fn unwrap(&self) -> &dyn ShapeTrait {
        match &self {
            Shape::Rec(rectangle) => rectangle,
            Shape::Tri(triangle) => triangle,
            Shape::Circ(circle) => circle,
        } 
    }
}


//Structs that implements ShapeTrait

//Rectangle struct
#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    lenght: f64,
}
impl ShapeTrait for Rectangle {

    //Functions that Rectangle implements as being shape (it has area and name)
    fn area(&self) -> f64 {
        self.width * self.lenght
    }
    fn shape_name(&self) -> String {
        String::from("rectangle with width ")+&(self.width.to_string())[..]+" and lenght "+&(self.lenght.to_string())[..]
    }
}
impl Rectangle {
    // function new that will create new instance of Rectangle struct
    pub fn new(width: f64, lenght: f64) -> Self {
        Self {
            width,
            lenght,
        }
    }
}

//Triangle struct
#[derive(Debug)]
pub struct Triangle {
    base: f64,
    height: f64,
}
impl ShapeTrait for Triangle {

    //Functions that Triangle implements as being shape (it has area and name)
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
    fn shape_name(&self) -> String {
        String::from("triangle with base ")+&(self.base.to_string())[..]+" and height "+&(self.height.to_string())[..]
    }
}
impl Triangle {
// function new that will create new instance of Triangle struct
    pub fn new(base: f64, height: f64) -> Self {
        Self {
            base,
            height,
        }
    }
}

//Circle struct
#[derive(Debug)]
pub struct Circle {
    radius: f64,
}
impl ShapeTrait for Circle {

    //Functions that Circle implements as being shape (it has area and name)
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14
    }
    fn shape_name(&self) -> String {
        String::from("circle with radius: ")+&(self.radius.to_string())[..]
    }
}

impl Circle {
// function new that will create new instance of Circle struct
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
        }
    }
}