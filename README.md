Here's some explaination regarding some of the design choices made in this project:

- the test coverage is ridiculous, but it's not the point here. Code has been
  splitted into files and small functions in a way that makes it easy to test.
- `AtomicBool` usage is questionable because when moving a rover to another
   cell, we first set the occupation status to ̀`true` for the destination cell,
   then to `false` for the previous cell. The whole process is not atomic and
   it can leads to sitation where a rover is seen as occupying two cells at a
   time. We could use a `Mutex` to lock both cells and update the statuses at
   the same time, but it would be overkill for this application.
- `pest` crate has been used, but we could have used regexes or `nom` for
  example.
- a Custom error type has been created to handle errors correctly instead of
  returning a `Result<T, Box<dyn Error>>` everywhere.
- getters and setters have been used and by default most of the structs
  fields are not public. This enforces users to call a construction method that
  can check values and return an error instead of letting the user building
  invalid objects.

# Build and run

```shell
cargo run --release -- ./data/input.txt
```

# Tests

```shell
cargo test --release
```

or

```shell
cargo nextest run --release
```
