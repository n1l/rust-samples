fn main() {
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    let mut y = 0;
    while ({y+=1; y} < 10) {
        println!("{}", y)
    }

    println!("The value of y is: {y}");
}