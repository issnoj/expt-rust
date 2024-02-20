# hello コマンド

## Dependencies
 
- [assert_cmd](https://github.com/assert-rs/assert_cmd)

## Usages

```bash
# echo Hello, world!
$ cargo run --bin hello
Hello, world!

# true
$ cargo run --bin true && echo $?
0

# false
$ cargo run --bin false || echo $?
Aborted
134
```