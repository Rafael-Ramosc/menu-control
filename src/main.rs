fn main() {
    println!("Welcome!");
    println!("Select a option:");
    println!("1. Hello World");
    println!("2. About ME");
    println!("3. EXIT");

    println!("Enter a option number:");
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();

    println!("You choose: {}", option);

    match option.trim().parse::<i32>().unwrap() {
        1 => println!("Hello World"),
        2 => println!("My name is Rafael Ramos"),
        3 => println!("Exiting..."),
        _ => println!("Invalid option!"),
    }
}
