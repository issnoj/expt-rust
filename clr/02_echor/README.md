# echo コマンド

## Dependencies

- [clap](https://github.com/clap-rs/clap) 
- [assert_cmd](https://github.com/assert-rs/assert_cmd)
- [predicates](https://github.com/assert-rs/predicates-rs)

## Help

```bash
$ cargo run -- -h
Usage: echor [OPTIONS] <TEXT>...

Arguments:
  <TEXT>...  Input text

Options:
  -n             Do not print newline
  -h, --help     Print help
  -V, --version  Print version
```

## Usages

```bash
# echo hello world
$ cargo run -- hello world
hello world

# echo -n hellow world
$ cargo run -- hello world -n
$ cargo run -- -n hello world
```