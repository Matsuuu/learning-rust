#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: i8
}

struct Pair(i32, i32);

struct Point {
    x: f32,
    y: f32
}

struct Coordinates {
    latitude: Point,
    longitude: Point
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    let name = String::from("Matsu");
    let age = 26;

    let matsu = Person {name, age};

    println!("{:?}", matsu);

    let rect = Rectangle { 
        top_left: Point {
            x: 12.0, y: 20.0
        },
        bottom_right: Point {
            x: 22.0, y: 8.0
        }
    };

    rect_area(rect);
}

fn rect_area(rect: Rectangle) -> i32 {
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;

    println!("Width: {}, Height: {}", width, height);
    return 0;
}
