fn main() {
    functions_and_scope();
    return_values_and_scope();
    referencing();
    muttable_referencing();
    muttable_and_immutable_referencing();
    try_dangling_reference();
}

fn functions_and_scope() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn referencing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn muttable_referencing() {
    let mut s = String::from("hello");

    change_str(&mut s);

    println!("{}", s)
}

fn change_str(str: &mut String) {
    str.push_str(", world");
}

fn muttable_and_immutable_referencing() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; //it does not compile because imuttable referencing in r1 and r2 was not yet used at this point

    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

fn try_dangling_reference() {
    // let reference_to_nothing = dangle(); //won't compile
    let reference_to_string = no_dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}