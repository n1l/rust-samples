fn main() {
    strings_compile();
    //does_not_compile();
}

fn strings_compile() {
    let s1 = "hello";
    let s2 = s1;

    println!("{} {}", s1, s2);
}

// fn strings_does_not_compile() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{} {}", s1, s2);
// }
