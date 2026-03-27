fn main() {
    // let pi = 3.14;
    // println!("pi value: {}", pi);
    // pi = 3.14000;
    // println!("updated pi value: {}", pi);
// The above code doesn't compiles
// Reason: In rust variables are immutable by default (So, pi value can't be assigned twice)
// Below is the correct implementation of above code
    let mut pi = 3.14;
    println!("pi value: {}", pi);
    pi = 3.14000;
    println!("updated pi value: {}", pi);
// Pro Tip: Always use mut to make a variable mutable.
// Rust ensures variables aren't editable by any other process unless its declared as mutable.
}