use text_io::read;

fn create_set_numbers() -> Vec<i32> {
    print!("Write the first number of your set: ");
    let start_number: i32 = read!();

    print!("Write the last number of your set: ");
    let end_number: i32 = read!();

    let mut set_of_numbers: Vec<i32> = Vec::new();

    for i in start_number..(end_number + 1) {
        set_of_numbers.push(i);
    }

    set_of_numbers
}

fn main() {
    println!("Hello, world!");
}
