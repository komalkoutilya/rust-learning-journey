// Refer files in Ch3-CommonProgrammingConcepts/datatypes/src/bin for detailed info about data types

/*
    Rust have two categories of data types
    1. Scalar Types:
        i.  Integer
            a.  Signed Integer [i8, i16, i32 (default), i64, i128, isize]
            b.  Unsigned Integer [u8, u16, u32 (default), u64, u128, usize]
        Integer literals: Hexadecimal[0x], Octal[0o], Binary[0b], Byte[b] (byte used for u8 only)

        Integer Overflow: In debug mode the integer overflow will raise panics at runtime
        But in release mode, the integer overflow will perform 2's complement wrapping by default

        ii. Float
            f32 and f64 (default) are the two types of float data types available in rust
            All floating types in Rust are signed (follows IEEE-754 standard)
        iii.Character
            Rust characters follow Unicode instead of ASCII code
            Char is 4 bytes size in the Rust

        iv. Boolean
            Uses bool as datatype specifier, takes value either true/ false
            Boolean in rust occupies 1 byte

    2. Compound Types:
        Compound types can group multiple values into one type. There are two primitive compound types in Rust
        i.  Tuples
            a.  A tuple can group multiple values of different scalar types into one compound type
            b.  Destructuring of tuple is a way to assign tuple values to individual variables in another tuple (See Example to Understand)
            c.  Elements of tuple can be accessed via a period (.) Ex: tuple_variable.index_val
            d.  Stored on Stack when size is known at compile time

        ii. Arrays
            a.  Unlike tuples, arrays in rust must have same data type
            b.  Arrays are of fixed length
            c.  Array elements are accessed via [] square braces with index specified
            d.  Stored on Stack when size is known at compile time
            e.  The rust will raise panics when there occurred Array index out of bounds.
*/

fn main() {
    println!("==============Unsigned Integers==============");
    let int_u8: u8 = 90; // decimal 90
    let int_u16: u16 = 0x77; //decimal 119
    let int_u32: u32 = 0o10; // decimal 8
    let int_u64: u64 = 0b110010; // decimal 50
    let int_unsignedbyte: u8 = b'B'; // decimal 66 (This representation will only applicable to u8 type)

    println!("u8: {}", int_u8);
    println!("u16: {}", int_u16);
    println!("u32: {}", int_u32);
    println!("u64: {}", int_u64);
    println!("u8 with byte representaion: {}", int_unsignedbyte);

    println!("==============Signed Integers==============");
    let int_i8: i8 = -90; // decimal -90
    let int_i16: i16 = -0x77; //decimal -119
    let int_i32: i32 = -0o10; // decimal -8
    let int_i64: i64 = -0b110010; // decimal -50
    // let int_signedbyte : i8 = -b''; will raise a panic, Why? because -b return an u8:ASCII (where ASCII range 0-127), so can't be negated without casting.
    let int_signedbyte: i8 = -(b'' as i8); // decimal -127 (This representation will only applicable to u8 type)

    println!("i8: {}", int_i8);
    println!("i16: {}", int_i16);
    println!("i32: {}", int_i32);
    println!("i64: {}", int_i64);
    println!("i8 with byte representaion: {} :: used casting to i8 from u8 as b'B' always return u8, and negation is not defined for u8", int_signedbyte);

    println!("==============Floating Points==============");
    let float_f32: f32 = 3.14;
    let float_f64: f64 = 2.9e8;
    println!("float_f32: {}", float_f32);
    println!("float_f64: {}", float_f64);

    println!("==============Characters==============");
    let char_a : char = 'అ';
    let char_m : char = 'మ';
    let char_ma : char = 'మ';
    let char_virama : char = '్';
    println!("Amma: {}{}{}{}", char_a, char_m, char_virama, char_ma);

    println!("==============Boolean==============");
    let bool_true : bool = true;
    let bool_false : bool = false;
    println!("bool_true: {}", bool_true);
    println!("bool_false: {}", bool_false);

    println!("==============Tuples==============");
    let tuple_var1 = (1,2,3);
    let tuple_var2 : (char, bool, u64) = ('&', false, 899849);
    let (char_tup , bool_tup, u64_tup) = tuple_var2;

    println!("tuple_var1: {:?}", tuple_var1);
    println!("tuple_var2: {:?}", tuple_var2);
    println!("char_tup: {}", char_tup);
    println!("bool_tup: {}", bool_tup);
    println!("u64_tup: {}", u64_tup);
    println!("tuple_var_char: {}", tuple_var2.0);
    println!("tuple_var_bool: {}", tuple_var2.1);
    println!("tuple_var_u64: {}", tuple_var2.2);

    println!("==============Arrays==============");
    let array1 : [i32; 3] = [1,2,3];
    let array2 = [100;6];
    println!("array1: {:?}", array1);
    println!("array2: {:?}", array2);
    println!("Array 1: 2nd element is {}", array1[1]);
}