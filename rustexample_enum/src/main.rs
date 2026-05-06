// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad, //pageload is a function in rust, so we need to use camel case for the enum variant
    PageUnload, //pageunload is also a function in rust, so we need to use camel case for the enum variant
    //a variant is like a tuple struct, but different variants in the same enum may have different types and amounts of values associated with them,
    // like tuple structs, but each variant can have different types and amounts of associated data.

    KeyPress(char), //KeyPress is a tuple struct variant, it has one field of type char and 
    //it is similar to KeyPress in JavaScript, but it is not the same as KeyPress in JavaScript because 
    //it is a tuple struct variant in Rust, while KeyPress in JavaScript is a function.

    Paste(String), // Paste is also a tuple struct variant, it has one field of type String 
    // and it is similar to Paste in JavaScript, but it is not the same as Paste in JavaScript
    // because it is a tuple struct variant in Rust, while Paste in JavaScript is a function.
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
