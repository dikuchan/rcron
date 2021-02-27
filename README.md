# rcron

## Requirements
* Nightly Rust
* UN*X

## Usage
```
$ git clone ...
$ cd rcron
$ cargo install --release
$ cargo run --release --bin daemon
...
$ cargo run --release --bin --client -- ...
```

## Example
```
$ cargo run --release --bin client -- --command='ls' --args='-a' --time='2021.12.31 23:59:59'
Scheduled: ok
```
