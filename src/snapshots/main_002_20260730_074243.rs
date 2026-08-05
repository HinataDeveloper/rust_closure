fn main() {
    println!("\n");

    let message = String::from("I am a beginner Rustacean ...");

    let my_closure = || {
        println!("message is: {}", message);
    };

    my_closure();

    println!("\nThe End ...");
}
