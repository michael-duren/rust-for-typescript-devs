Terminology 
Dropped - releasing Memory

There are **THREE** rules you must have in your head at all times
1. There can only be **one** value owner
2. There can be **unlimited** immutable borrows (reference) with **no** mutable references
3. There can be only **one** mutable reference and **no** immutable reference

There is one rule for Lifetimes
1. A reference cannot outlive it's value


Stated differently 
One var owns the data
One var can change the data
Many vars can look at the data
You cannot look and change the data simultaneously
You cannot refer to something that has been dropped (released in memory)

```rust
#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    items
        .iter() // iterate on each item
        .for_each(|i| println!("{:?}", i));
}

fn main() {
    let mut item = Item { count: 1 };

    println!("{:?}", item);

    add_one(&mut item);
    println!("{:?}", item);

    let mut items: Vec<Item> = vec![Item { count: 3 }, item];

    print_all(&items);

    let first = items.get_mut(0);
    let second = items.get_mut(1);
}
```
