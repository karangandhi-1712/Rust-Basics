// fn main() {
//     println!("Hello, world!");
//     let x = another_function(42, 23);
//     println!("{:?}", x);

//     let x = false;
//     let number = if x { 5 } else { 6 };

//     println!("{}", number);
// }

// fn another_function(number: i32, number2: i32) -> (i32, i32) {
//     println!("This is another function.");
//     return (number + number2, number - number2);
// }

fn main() {
    let i = 5;
    call_int(i);
    println!("AFTER CALLING THE FUNCTION, the value of i: {}", i);

    let s = String::from("Hello");
    let sx = call_string1(s);
    // M1: ownership moved into function and returned back, so we reassign to s
    // Now s is valid again because ownership was returned above.
    // We can safely borrow it here without transferring ownership.
    call_string2(&sx); // M2: passing a reference (no ownership move), see below for why & symbol is used here
    println!("AFTER CALLING THE FUNCTION, the value of s: {}", sx);
}

// call int function
fn call_int(i: i32) {
    println!("call_int i: {}", i);
}

// // call string function
// fn call_string(s: String) {x
//     println!("call_string s: {}", s);
// }

//this code will not compile because the ownership of the string is moved to the call_string function,
//and after that, we cannot use the variable s in the main function. 
//To fix this, we can either return the string from the call_string function or pass a reference

//M1 : take and give back ownership of the string
// fn call_string(s: String) -> String {
//     println!("call_string s: {}", s);
//     s
// }   


//M2 : pass a reference to the string
// fn call_string(s: &String) {
//     println!("call_string s: {}", s);
// }

//M1
fn call_string1(s: String) -> String {
    println!("call_string s: {}", s);
    s
}

// M2
fn call_string2(s: &String) {
    println!("call_string s: {}", s);
}
