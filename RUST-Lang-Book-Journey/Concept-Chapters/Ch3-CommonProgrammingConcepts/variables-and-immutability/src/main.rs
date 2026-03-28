fn main() {
    let pi = 3.14;
    println!("pi value: {}", pi);
    pi = 3.14000;
    println!("updated pi value: {}", pi);
// The above code doesn't compiles
// Reason: In rust variables are immutable by default (So, pi value can't be assigned twice)
// Below is the correct implementation of above code
    let mut pi = 3.14;
    println!("pi value: {}", pi);
    pi = 3.14000;
    println!("updated pi value: {}", pi);

// Constants in rust are defined via const keyword
// Constants are always immutable, and are always bound to a datatype declared during declaration
// The datatype is mandatory during declaring a constant
    const SPEED_OF_LIGHT: f64 = 3e8;
    println!("The speed of light: {}", SPEED_OF_LIGHT);

// Shadowing of variables
    let svar =  49;
    println!("The svar value: {}", svar);
    let svar = svar + 1;
    println!("updated svar value: {}", svar);
    {
        let svar = "string value";
        println!("Inner scope svar value: {}", svar);
    }
    println!("original svar value: {}", svar);

//===============================PRO TIPS===============================
// Use mut to make a variable mutable.
// Rust ensures variables aren't editable by any other process unless its declared as mutable.
// Constants are valid for the entire execution of the program within its scope.
// Shadowing enables us to modify variable without making it mutable.
// How Shadowing differ from mut??
//     Shadowing allows to re-declare a variable (So, we can change the type of variable)
//     Mutability (mut) can only change variable value, but not data type
//     Shadowing is capable of altering variable type as well
}