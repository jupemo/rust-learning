// Attributes are basically named tuples

// Attribute to hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Unit;

struct Pair(i32, f32);


#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as fields of another struct

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
     let Rectangle {top_left: Point {x: x1, y:y1}, bottom_right: Point{x: x2, y: y2}} = rectangle;
    (x1 - y1) * (x2 - y2)
}

fn square(point: Point, with: f32) -> Rectangle {
   Rectangle {top_left: Point {x: point.x, y: point.y }, bottom_right: Point {x: with, ..point}}
}

fn main() {

    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    // Instantiate a point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    //Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using let binding
    let Point { x: left_edge, y: top_edge } = point;


    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    println!("rectangle points ({}, {}), ({}, {})", _rectangle.top_left.x, _rectangle.top_left.y,
             _rectangle.bottom_right.x, _rectangle.bottom_right.y);


    // Instantiate a unit struct
    let _unit = Unit;
    println!("unit {:?}", _unit);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle area {}", rect_area(&_rectangle));

    println!("square {:?}", square(Point{x: 12.5, y:3.2}, 5.0))

}
