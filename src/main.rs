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

fn binary_search(set: Vec<i32>, target: i32) -> (Option<usize>, i32) {
    let mut low = 0;
    let mut high = set.len() - 1;

    let mut attempts: i32 = 0;

    while low <= high {
        let mid = (high + low) / 2;

        if set[mid] == target {
            attempts = attempts + 1;
            return (Some(mid), attempts);
        } else if set[mid] < target {
            attempts = attempts + 1;
            low = mid + 1;
        } else {
            attempts = attempts + 1;
            high = mid - 1;
        }
    }

    (None, attempts)
}

fn main() {
    let set_numbers = create_set_numbers();
    print!("Write target number: ");
    let target_number: i32 = read!();

    let test_binary_search = binary_search(set_numbers, target_number);

    println!("{:?}", test_binary_search);
}
