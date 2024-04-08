fn main() {
    test_for_loop();
    loop_with_break();
    tagged_loop();
}

fn test_for_loop() {
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in (0..10).step_by(2) {
        println!("{}", a[i]);
    }
}

fn loop_with_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn tagged_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
