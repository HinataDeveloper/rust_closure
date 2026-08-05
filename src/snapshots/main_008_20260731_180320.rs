fn main() {
    println!("\n");

    let send_this_message = String::from("an apple a day keeps the doctor away");

    let message = move || {
        let mut msg = send_this_message;
        msg.push_str(" - Loaded");
        msg
    };

    execute_action(message);

    println!("\nThe End ...\n");
}

fn execute_action<F>(action: F)
where
    F: FnOnce() -> String,
{
    println!("starting execution: ");
    let result = action();
    println!("value of result is: {}", result);
}

