fn main() {
    println!("\n");
    variables();
}

fn variables() {
    /* Variable */

    // immutable variable
    let a: u8 = 1;
    println!("a = {a} is immutable!\na = 2 \\\\ throw Error!");

    // mutable variable
    let mut b: u8 = 2;
    println!("b = {b} is mutable!");

    b = 3;
    println!("now b = {b}, wont throw Error");

    /* Constant */

    // it's immutable and the type of the value must be annotated.
    const NAME: &str = "Romeo Noveanre";
    // const age: u8 = 18; // must be UPPERCASE
    const AGE: u8 = 18;


    println!("Hi, my name is {NAME}, and im {AGE} y/o.");

    /* Shadowing  */
    // you can declare a new variable with the same name as a
    // previous variable. Rustaceans say that the first variable is
    // shadowed by the second, which means that the second variable
    // is what the compiler will see when you use the name of the
    // variable. In effect, the second variable overshadows the first,
    // taking any uses of the variable name to itself until either it
    // itself is shadowed or the scope ends. We can shadow a variable
    // by using the same variable’s name and repeating the use of the let keyword
    let x: u8 = 5;
    let x: u8 = x + 1;

    {
        let x: u8 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
