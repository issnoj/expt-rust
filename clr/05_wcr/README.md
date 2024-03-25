# wc コマンド

## Dependencies

- [clap](https://github.com/clap-rs/clap) 
- [assert_cmd](https://github.com/assert-rs/assert_cmd)
- [predicates](https://github.com/assert-rs/predicates-rs)
- [rand](https://github.com/rust-random/rand)

## Help

```bash
$ cargo run -- -h
Usage: wcr [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Input file(s) [default: -]

Options:
  -l, --lines    Show line count
  -w, --words    Show word count
  -c, --bytes    Show byte count
  -m, --chars    Show character count
  -h, --help     Print help
  -V, --version  Print version
```

## Usages

```bash
# wc tests/inputs/fox.txt
$ cargo run -- tests/inputs/fox.txt
       1       9      48 tests/inputs/fox.txt

# cat tests/inputs/fox.txt | wc
$ cat tests/inputs/fox.txt | cargo run
       1       9      48

# wc tests/inputs/fox.txt -lm
$ cargo run -- tests/inputs/fox.txt -lm
       1      48 tests/inputs/fox.txt

# wc tests/inputs/*.txt
$ cargo run -- tests/inputs/*.txt
       4      29     177 tests/inputs/atlamal.txt
       0       0       0 tests/inputs/empty.txt
       1       9      48 tests/inputs/fox.txt
       5      38     225 total
```