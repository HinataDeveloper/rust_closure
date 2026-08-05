fn main() {
    println!("\n");

    let message = String::from("What a good language programming Rust is!!");

    let closure_message = || {
        // here message variable borrowed to the println! macro
        println!("closure message: {}", message);
    };

    closure_message();

    println!("main message: {}", message);

    println!("\nThe End ...");
}
