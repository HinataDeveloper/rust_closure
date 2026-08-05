//! ...

fn main() {
    println!("\n");

    let my_config_type = 7_u8;

    let my_action = optional_config(my_config_type);

    my_action();

    println!("\nThe End ...\n");
}

fn optional_config(config_type: u8) -> Box<dyn FnOnce()> {
    if config_type == 1 {
        Box::new(move || println!("This is config type one"))
    } else if config_type == 2 {
        Box::new(move || println!("This is config type two"))
    } else if config_type == 3 {
        Box::new(move || println!("This is config type three"))
    } else {
        Box::new(move || println!("This is other config type ..."))
    }
}
