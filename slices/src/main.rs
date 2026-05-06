//Ranges shortcut for slices
fn main() {
let s = String:: from("Francesco");
// shortcut for initial index
let slice = &s[0..3];
println!("{}", slice);
let slice = &s[..3];
println!(" {}", slice);
}
//If you want to start from the beginning of a string, you can omit the initial index and just use ..3.
//This will give you a slice that starts from the beginning of the string and ends at index 3 (exclusive). The output will be "Fra".