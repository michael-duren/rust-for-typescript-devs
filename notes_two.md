### Error Handling

With Match Statement    

```rust
match a_function_that_can_error() {
    Ok(val) => println!("Yeah, yay {}", val),
    Err(e) => eprintln!("ohh no....{}", e)
}
```

```rust
if let Ok(value) = a_function_that_can_error() {
// something with the value
}
```

You don't care about the error (use sparingly)
```rust
_ - a_function_that_can_error()
```

YOLO
```rust
let foo = a_function_that_can_error().unwrap();
```

Respectful YOLO
```rust
let foo = a_function_that_can_error().expect("should never fail");
```

Defaults
```rust
let foo = a_function_that_can_error().unwrap_or(0);
```

Convert to option
`Ok(V) => Some(V)`
`Err(E) => None`
bai felicia

```rust
let foo = a_function_that_can_error().ok();

let foo = a_function_that_can_error()
    .map(|value| value + 1);

let foo = a_function_that_can_error()
    .and_then(|value| another_possible_error(value))
    .and_then(|value| again(value));
```

If your function returns an error you can do this
```rust
let foo = a_function_that_can_error()?;
```

### Side Notes On Errors

- thiserror - great for creating your own errors. should be used in libraries
- anyhow - great for applications

```rust
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
```
