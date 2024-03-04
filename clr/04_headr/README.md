# head コマンド

## Dependencies

- [clap](https://github.com/clap-rs/clap) 
- [assert_cmd](https://github.com/assert-rs/assert_cmd)
- [predicates](https://github.com/assert-rs/predicates-rs)
- [rand](https://github.com/rust-random/rand)

## Help

```bash
$ cargo run -- -h
Usage: headr [OPTIONS] [files]...

Arguments:
  [files]...  Input file(s) [default: -]

Options:
  -n, --lines <LINES>  Number of lines [default: 10]
  -c, --bytes <BYTES>  Number of bytes
  -h, --help           Print help
  -V, --version        Print version
```

## Usages

```bash
# head tests/inputs/ten.txt -n 3
$ cargo run -- tests/inputs/fox.txt
one
two
three

# cat tests/inputs/fox.txt | cat
$ cargo run -- tests/inputs/ten.txt -n 3
The quick brown fox jumps over the lazy dog.

# cat < tests/inputs/fox.txt
$ cargo run -- tests/inputs/*.txt aaaa -n 1
==> tests/inputs/empty.txt <==

==> tests/inputs/one.txt <==
Öne line, four words.

==> tests/inputs/ten.txt <==
one

==> tests/inputs/three.txt <==
Three

==> tests/inputs/two.txt <==
Two lines.
aaaa No such file or directory (os error 2)

# head tests/inputs/*.txt -c 2
$ cargo run -- tests/inputs/*.txt -c 2
==> tests/inputs/empty.txt <==

==> tests/inputs/one.txt <==
Ö
==> tests/inputs/ten.txt <==
on
==> tests/inputs/three.txt <==
Th
==> tests/inputs/two.txt <==
Tw
```