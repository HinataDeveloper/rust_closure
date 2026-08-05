fn main() {
    println!("\n");

    let my_message = "You should know, I am learning Zig and Rust.";

    let my_action = move || {
        my_message.to_string() + " - Loaded"
    };

    raphael(my_action);

    println!("\nThe End ...\n");
}

fn raphael<F>(action: F)
where
    F: FnOnce() -> String,
{
    println!("I am starting your action:");
    let resultant = action();
    println!("message is: {}", resultant);
}
