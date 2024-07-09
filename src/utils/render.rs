//i choosed to just calculate the terminal if the number is poistive, may change later
//todo: criar um menu para tools, como essa por exemplo
pub fn calculate_column(mut column_size_min: u32, column_size_max: u32) {
    let mut index: u32 = 0;

    while column_size_max > index {
        print!("-",);
        column_size_min += 1;
        index += 1;
    }
    //todo: criar uma parametro para controler o print abaixo
    // println!("");
    // println!("Column size: {}", column_size_min);
}
