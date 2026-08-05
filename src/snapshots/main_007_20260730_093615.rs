fn main() {
    println!("\n");

    execute_action(move || "I am beginner Rustacean".to_string());

    println!("\nThe End ...\n");
}

fn execute_action<F>(action: F)
where
    F: FnOnce() -> String,
{
    println!("Starting execution ...");
    let result = action();
    println!("result obtained: {}", result);
}
