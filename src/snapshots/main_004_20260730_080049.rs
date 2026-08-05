fn main() {
    println!("\n");

    let message = String::from("my function is studying forever.");

    let closure_message = || {
        let msg = message;
        println!("closure message is: {}", msg);
    };

    closure_message();

    // encounter with compile time error. because
    // message variable had moved in closure block.
    // println!("main message is: {}", message);

    println!("\nThe End ...\n");
}
