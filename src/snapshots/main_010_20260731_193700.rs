//! returning FnOnce from function

fn main() {
    println!("\n");

    let my_config = String::from("port: 3265 user: Raphael password: 321654");

    let resultant = create_initilizer(my_config);
    resultant();

    println!("\nThe End ...\n");
}

fn create_initilizer(config: String) -> impl FnOnce() {
    move || {
        println!("initializing system with: {}", config);
    }
}
