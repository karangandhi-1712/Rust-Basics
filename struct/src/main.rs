// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
//this is added to make the struct printable with {:?} in println!

struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
    top_right: Point,
    bottom_left: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    //what is happening here is that we are creating a new Point struct called bottom_right,
    //and we are using the x value from the literal 10.3, and for the y value, we are using the y value from another_point,
    //which is 0.2. This is done using the struct update syntax with ..another_point.
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // destructuring is a way to break apart a struct into its individual fields and assign them to variables.
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
        top_right: Point {
            x: 10.3,
            y: 0.4,
        },
        bottom_left: Point {
            x: 5.2,
            y: 0.2,
        },
    };

    // Instantiate a unit struct
    // to instantiate means to create an instance of a struct. In this case, we are creating an instance of the Unit struct, which is a unit struct that has no fields.
    //We can create an instance of it by simply writing Unit, and we can assign it to a variable, such as _unit.
    //The underscore at the beginning of the variable name is a convention in Rust
    // to indicate that the variable is intentionally unused, which helps to avoid compiler warnings about unused variables.
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    // Now we can use the variables `integer` and `decimal` to access the values from the tuple struct.
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle_area = (_rectangle.bottom_right.x - _rectangle.top_left.x) * (_rectangle.top_left.y - _rectangle.bottom_right.y); //destructuring the rectangle to get the area of the rectangle by using the formula (x2 - x1) * (y1 - y2)
    println!("The area of the rectangle is: {}", rectangle_area);
}
