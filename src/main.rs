fn main() {
    conditions();
    loops();
    parameter("Hey");
}

fn conditions() {
    if (1 > 2) {
        println!("hello from if condition");
    } else {
        println!("hello from else condition");
    }
}

fn loops() {
    let mut i: i16 = 0;

    /*while i < 5 {
        i += 1;
        println!("hello from while loop");
    }*/

    for i in 1..=5 {
        println!("hello from for loop");
    }
}

fn parameter(value: &str) {
    println!("hello from parameter, value: {}", value);
}