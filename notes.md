## To begin with, there are 2 things that have to be understood in Rust

1. Iterators
2. Enums


**Iterators**
vec![1, 2, 3] - an iterator is a seperate data structure that can walk through that data structure

Iterator way of thinking
Not in javascript
`[Type] -> [Iterator] -> [Type]`
This typically gives us code that looks like
```rust
some_type
    .iter() // creates iterator
    .filter(|x| x + 1) ...
    ) // a collection of combinators
    .collect/sum/count/for_each() // some operation that takes the iterator and converts it to type
```

exercise
```rust
    let data = vec![1, 2, 3];
    let mut x = data
        .iter() // iterator refers to vector
        .map(|num| num + 1); // recieves iterator and the value that it recieves passes to lambda func

    let mut new_vector = vec![];

    while let Some(x) = x.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector); // pretty print
```



more examples for iterating in rust
```rust
    let file = std::fs::read_to_string("lines").unwrap();

    file // is now a string of the contents of the file
        .lines() // similar to split creates an iterator for us
        .enumerate() // get every index
        .filter(|(i, _)| i % 2 != 0)
        .skip(2) // literally skip two from arr
        .take(2) // literally take two from arr
        .for_each(|(_, line)| println!("{}", line));
```


### Enums and struct

```rust
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Blue => true,
            Color::Green => false,
            Color::Yellow => true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Green => println!("green"),
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}
```

### Options in Rust

the Enum definition of Option
```rust
enum Option<T> {
    None
    Some(T),
}
```

### Javascript

Javascript code for rust examples

```typescript
import fs from "fs";

// const list = [1, 2, 3];

// const added = list.map((item) => item + 1);

// console.log(added);

fs.readFileSync("lines")
  .toString()
  .split("\n") // takes entire string reads character by charcter
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4)
  .forEach((line) => console.log(line));

enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}

function printColor(color: Color) {
  switch (color) {
    case Color.Green:
      console.log("green");
      break;
    case Color.Red:
      console.log("red");
      break;
    case Color.Blue:
      console.log("blue");
      break;
  }
}

printColor(Color.Yellow);

type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

const append = (items: Item[]) => {
  items.push("Hello Fem!");
};

const itemsArr: Item[] = ["Hello", { age: 10, name: "Mason" }];

console.log(itemsArr);
append(itemsArr);
console.log(itemsArr);

const numbers: number[] = [];

append(numbers);
```

```rust
struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

// items which is a mutable reference to a list of items
fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello, Fem".to_string()));
}
```


MORE RUST CODE

```rust

fn do_something_with_number(num: Option<isize>) -> Option<isize> {
    return Some(num? * 5);
    // return num.map(|x| x * 5);
}

fn practice(nums: Vec<usize>, index: usize) -> usize {
    // match Some(nums[index]) {
    //     Some(n) => n * 5,
    //     None => index * 5,
    // }

    nums.get(index).unwrap_or(&index) * 5
}

```


