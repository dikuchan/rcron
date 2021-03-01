# rcron

## Requirements
* Nightly Rust
* UN*X

## Usage
```
$ git clone ...
$ cd rcron
$ cargo install --release
(as root) # cargo run --release --bin daemon
...
$ cargo run --release --bin --client -- ...
```

## Examples
```
$ cargo run --release --bin client -- --command='ls' --args='-a' --time='2021.12.31 23:59:59'
Scheduled: ok
```

```
$ cargo run --release --bin client -- -c='echo' -a='$HOME' -t='2022.01.01 00:00:00'
Scheduled: ok
```

```
$ cat /var/log/rcron.log
[00:00:00.000] (7f63b5ea3c00) INFO   Daemon launched
[00:00:09.053] (7f63b5ea3c00) INFO   Added job 'ls'
[00:00:16.842] (7f63b5ea3c00) INFO   Added job 'echo'
```
