fn main() {
    println!("\n");
    // variables();
    // data_types();
    // functions();
}

// fn variables() {
//     /* Variable */

//     // immutable variable
//     let a: u8 = 1;
//     println!("a = {a} is immutable!\na = 2 \\\\ throw Error!");

//     // mutable variable
//     let mut b: u8 = 2;
//     println!("b = {b} is mutable!");

//     b = 3;
//     println!("now b = {b}, wont throw Error");

//     /* Constant */

//     // it's immutable and the type of the value must be annotated.
//     const NAME: &str = "Romeo Noveanre";
//     // const age: u8 = 18; // must be UPPERCASE
//     const AGE: u8 = 18;


//     println!("Hi, my name is {NAME}, and im {AGE} y/o.");

//     /* Shadowing  */
//     // you can declare a new variable with the same name as a
//     // previous variable. Rustaceans say that the first variable is
//     // shadowed by the second, which means that the second variable
//     // is what the compiler will see when you use the name of the
//     // variable. In effect, the second variable overshadows the first,
//     // taking any uses of the variable name to itself until either it
//     // itself is shadowed or the scope ends. We can shadow a variable
//     // by using the same variable’s name and repeating the use of the let keyword
//     let x: u8 = 5;
//     let x: u8 = x + 1;

//     {
//         let x: u8 = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn data_types() {
//     /*
//     Integer:
//         I/U 8 bytes -> 128 bytes depends on the architecture
//         n = 2^n bytes
//      */

//     let my_age: u8 = 18;

//     /*
//     Floating point:
//         f32 & f64.
//         f64 by default
//      */

//     let pi: f32 = 3.14;

//     /*
//     Rust supports the basic mathematical operations you’d
//     expect for all the number types: addition, subtraction,
//     multiplication, division, and remainder. Integer division
//     truncates toward zero to the nearest integer. The following
//     code shows how you’d use each numeric operation in a let statement:
//      */

//     // let sum = pi + my_age; somehow it's error
//     // println!("{sum}");

//     /*
//     Boolean
//         who doesn't know boolean?
//      */
//     let is_me_dumb: bool = true;

//     /*
//     Char
//         glad for learn SQL first, though my first language was Pascal
//         it's one character data types btw
//      */
//     let c = 'z';
//     let z: char = 'z'; // with explicit type annotation
//     let heart_eyed_cat = '😻';

//     // Let's be more serious

//     /*
//     Tuple
//         I remember first knowing tuple exist was from sololearn on python course
//         ** A tuple is a general way of grouping together a number
//         of values with a variety of types into one compound
//         type. Tuples have a fixed length: once declared, they
//         cannot grow or shrink in size. **
//      */
//     let my_tuple: (&str, u8, f64) = ("Romeo", 18, 3.14);

//     // Tuple destructuring
//     let (name, age, pee_as_pi) = my_tuple;

//     // accessing tuple
//     let my_age = my_tuple.1;

//     /*
//     Array
//         in Rust, every array items must have the same type
//      */
//                 //[type; length]
//     let my_array: [i32; 3] = [1, 2, 3];
//     let arr_str: [&str; 2] = ["1", "2"];

//     // accessing array
//     let first_arr = my_array[0];

//     // what? it's done.
//     // WHERE IS DICTIONARY !!!

// }

// fn functions() {
//         let y = {
//             let x = 3;
//             // return
//             x + 1
//     };

//     println!("The value of y is: {y}");

//     let s = sum(5, 6);

//     println!("sum of 5 + 6 = {s}");

//     fn sum(a: i32, b: i32) -> i32 {
//         // return without return keyword, ommit ;
//         a + b
//     }
// }

