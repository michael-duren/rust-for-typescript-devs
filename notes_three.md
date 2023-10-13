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

Iterator
An iterator isn't just something we interact with, it's somthing we can also create
Implement an iterator for Rectangle. It will iterate over the four points
- struct RectIter with a points: Vec<(f64, f64)> and idx: usize
- implement Iterator for RectIter
- implement IntoIterator for Rectangle `src/main.rs`
- create a rect
- iteratove over a rect for `point` in rect printing out each point
- print out the entire rectangle via the Display trait

```rust
impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        return self.points.get(idx).map(|x| *x); // * will copy out the value if it implements the copy trait
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return RectIter {
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}
```

### IntoIterator
It *consumes* the thing you give it, we need to give it something to consume that won't consume our original struct

