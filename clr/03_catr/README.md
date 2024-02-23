# echo コマンド

## Dependencies

- [clap](https://github.com/clap-rs/clap) 
- [assert_cmd](https://github.com/assert-rs/assert_cmd)
- [predicates](https://github.com/assert-rs/predicates-rs)
- [rand](https://github.com/rust-random/rand)

## Help

```bash
$ cargo run -- -h
Usage: catr [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Input file(s) [default: -]

Options:
  -n, --number           Number lines
  -b, --number-nonblank  Number non-blank lines
  -h, --help             Print help
  -V, --version          Print version
```

## Usages

```bash
# cat tests/inputs/fox.txt
$ cargo run -- tests/inputs/fox.txt
The quick brown fox jumps over the lazy dog.

# cat tests/inputs/fox.txt | cat
$ cat tests/inputs/fox.txt | cargo run
The quick brown fox jumps over the lazy dog.

# cat < tests/inputs/fox.txt
$ cargo run -- - < tests/inputs/fox.txt
The quick brown fox jumps over the lazy dog.

# cat -b tests/inputs/the-bustle.txt
$ cargo run -- -b tests/inputs/the-bustle.txt
     1  The bustle in a house
     2  The morning after death
     3  Is solemnest of industries
     4  Enacted upon earth,

     5  The sweeping up the heart,
     6  And putting love away
     7  We shall not want to use again
     8  Until eternity.

# cat -n tests/inputs/*.txt
$ cargo run -- -n tests/inputs/*.txt
     1  The quick brown fox jumps over the lazy dog.
     2  Do not worry, spiders,
     3  I keep house
     4  casually.
     5  The bustle in a house
     6  The morning after death
     7  Is solemnest of industries
     8  Enacted upon earth,
     9
    10  The sweeping up the heart,
    11  And putting love away
    12  We shall not want to use again
    13  Until eternity.
```