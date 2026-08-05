fn main() {
    println!("\n");

    let message = String::from("an apple a day keeps the doctor away.");

    let closure_message = || {
        let msg = message;
        println!("closure message is: {}", msg);
    };

    closure_message();

    // encounter with compile time error. because
    // message variable has moved on closure block. and 
    // after first call of 'closure_message' it is dropped.
    closure_message();


    println!("\nThe End ...\n");
}
