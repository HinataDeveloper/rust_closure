fn main() {
    println!("\n");

    let my_number = 120;

    let my_closure = move || {
        let my_num = my_number;
        println!("my number is: {}", my_num)
    };

    // we using move keyword to capture outer variables
    // but as normal behavior as always has copied into the closure.
    // so there is no problem to call my_closure more than once.
    my_closure();
    my_closure();

    println!("\nThe End ...\n");
}
