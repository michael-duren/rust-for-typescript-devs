fn main() {
    let file_path = std::env::args().nth(1).expect("The file name to exist");

    // match std::fs::read_to_string(file_path) {
    //     Ok(file_contents) => println!("{}", file_contents),
    //     Err(e) => eprintln!("Oh no you dumb ass {}", e),
    // }

    let file = std::fs::read_to_string(file_path).expect("File name doesn't exist");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Not a number");
        }
    });
}
