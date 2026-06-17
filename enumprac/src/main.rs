// #[derive(Debug)]
// #[allow(dead_code)]
// enum Color {
// // A variant that holds a tuple of three 8-bit unsigned integers
// Rgb(u8, u8, u8),
// // A variant that holds a single String
// Named(String),
// }
// fn main() {
// let red = Color::Rgb(255, 0, 0);
// let custom_color = Color::Named(String::from("Forest Green"));
// println!("An RGB color: {:?}", red);
// println!("A named color: {:?}", custom_color);
// }


// // fn main()
// // {
// //     //trying shadowing
// //     let x = 5;
// //     //x is immmutable by default, but we can shadow it with a new value
// //     let x = x + 1; // this shadows the previous x with a new value
// //     println!("The value of x is: {}", x); // this will print 6 and not 5.
// //     //now i will put x=7.
// //     let x = 7; // this didnt cause an error because we are shadowing the previous x with a new value.
// //     println!("The value of x is: {}", x);
// // }


enum Message {
Quit,
Move { x: i32, y: i32 },
Write(String),
ChangeColor(i32, i32, i32),
}
impl Message {
fn call(&self) {
match self {
Message::Quit => println!("Quit message"),
Message::Move { x, y } => println!("Move to x: {}, y: {}", x,
y),
Message::Write(text) => println!("Write message: {}", text),
Message::ChangeColor(r, g, b) => println!("Change color to
red: {}, green: {}, blue: {}", r, g, b),
}
}
}

// The main function is the entry point of the program
fn main() {
    // Instantiate the Quit variant
    let m_quit = Message::Quit;
    
    // Instantiate the Move variant with x and y coordinates
    let m_move = Message::Move { x: 10, y: 20 };
    
    // Instantiate the Write variant with a String message
    let m_write = Message::Write(String::from("Hello, enums!"));
    
    // Instantiate the ChangeColor variant with RGB tuple values
    let m_color = Message::ChangeColor(255, 0, 0);

    // Invoke the call method on each message instance
    m_quit.call();
    m_move.call();
    m_write.call();
    m_color.call();
}