fn main() {
    println!("\n");

    let my_name = String::from("Raphael");
    let print_prefix = |my_name: String| {
        println!("name is: {}", my_name);
    };

    print_prefix(my_name);

    println!("\nThe End ...\n");
}
