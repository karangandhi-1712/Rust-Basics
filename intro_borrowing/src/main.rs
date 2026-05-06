//M1 - for context refer to hello

// fn main()
// {
// // Return Values and Scope
//     let s1= String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println! ("The length of '{}' is {}.", s2, len);
// }
// //calculate length of a string
// fn calculate_length(s: String) ->(String, usize)
// {
//     let length = s.len();
//     (s, length)
// }

// M2 - for context refer to hello
// fn main() {
// let s1 = String::from("hello");
// let len = calculate_length (&s1); //passing reference to the string
// println! ("The length of '{}' is {}.", s1, len);
// }
// //calculate length of a string
// fn calculate_length(s: &String) -> usize {
// let length = s.len();
// length
// }
// In this code, we are using references to borrow the string instead of taking ownership of it.
// The calculate_length function takes a reference to a String (&String) and returns its length as a usize.
// This way, we can use the original string (s1) after calling the function without any issues, as we are not transferring ownership.

// Change borrowed value
// fn main() {
//     let mut s = String::from("hello");
//     change_borrowed_value(&mut s);
//     println("{}", s);
// }
// //fn to change borrowed value
// fn change_borrowed_value(s: &mut String) {
//     s.push_str(" world");
// }

//multiple mutable references
// fn main()
// {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s; //error: cannot borrow `s` as   mutable more than once at a time
//     println!("{}, {}", r1, r2);
// }
// so we have to use only one mutable reference at a time.
// We can create a new scope to allow multiple mutable references, but they cannot be used simultaneously.
//eg:
// fn main()
// {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s; // first mutable reference
//         r1.push_str(" world");
//         println!("r1: {}", r1);
//     } // r1 goes out of scope here, so we can create a new mutable reference
//     let r2 = &mut s; // second mutable reference
//     r2.push_str(" again");
//     println!("r2: {}", r2);
// }

// Mutable and immutable references
// fn main() {
// let mut s = String::from("hello");
// let s1 = &s; //immutable reference
// let s2 = &s; // immutable reference
// let s3 =&mut s; // mut ble reference
// println("{}, {}, {}", s, s2, s3);
// }
// This code will not compile because we cannot have a mutable reference (s3) while there are immutable references (s1 and s2) to the same data (s).
// To fix this, we need to ensure that we do not have any immutable references when we want to create a mutable reference. We can do this by creating a new scope for the immutable references, like this:
fn main() {
    let mut s = String::from("hello");
    let s1 = &s; // immutable reference
    let s2 = &s; // immutable reference
    println!("{}, {}", s1, s2);
    // now we can create a mutable reference after the immutable references are no longer used

    let s3 = &mut s; // mutable reference
    s3.push_str(" world");
    println!("{}", s3);
}

//dangling references -- see from video francesco no.7/53 ending


