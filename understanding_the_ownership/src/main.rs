fn main() {
    // println!("Hello World");

    // {
    //     let var = "rustlang";
    //     println!("var is {}", var);
    // }
    // println!("var is {}", var);    // var is out of scope here

    // // let s1 = "rustlang";    // the string literal declaration will work
    // let s1 = String::from("rustlang");
    // let s2 = s1;
    // println!("s1 is {} and s2 is {}", s1, s2);

    // let s = String::from("hello");
    // change(&s);

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
    
}


// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


