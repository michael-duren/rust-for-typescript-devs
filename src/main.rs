fn main() {
    // let data = vec![1, 2, 3];
    // let items = data.iter().map(|x| x + 1);

    // println!("{:?}", items);
    assert_eq!(10, cooking_time(10));
}

fn cooking_time(eggs: u32) -> u32 {
    let boiled_sets: u32 = eggs / 8 * 5;
    let remainder: u32 = if eggs % 8 == 0 { 0 } else { 5 };

    return boiled_sets + remainder;
}
