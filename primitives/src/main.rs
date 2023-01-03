
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;

    let tup = (12, 12.5, "foo");

    let numbers = [1,2,3,4,5,6];
    println!("Array access - [0]: {}", numbers[0]);
    println!("Array access - [5]: {}", numbers[5]);

    analyze_slice(&numbers);

    let num = numbers.get(2);
    match num {
        Some(numVal) => println!("NUM {}", numVal),
        None => todo!(),
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    return (bool_param, int_param);
}

fn analyze_slice(slice: &[i32]) {
    println!("The first element is: {}", slice[0]);
    println!("Length of slice is: {}", slice.len());
}
