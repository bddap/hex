# hex

Converts stdin to or from hexadecimal.

## Use:

```
$ echo hey there | hex to
6865792074686572650a
```

```
$ echo -n 6865792074686572650a | hex from
hey there
```

```
$ echo hey there | hex to | hex to | hex from | hex from
hey there
```

## Installation

1. You'll need the [rust compiler](https://www.rust-lang.org/en-US/install.html).
2. Clone this repo, then run `cargo install`.
