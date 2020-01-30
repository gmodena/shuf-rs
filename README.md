![](https://github.com/gmodena/shuf-rs/workflows/Build%20and%20test/badge.svg)

# shuf-rs

Shuffle lines of text with reservior sampling.

# Build

```
cargo build
```

The resulting binary is found under `target`.

# Usage

```
USAGE:
    shuf-rs [OPTIONS] [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --head-count=COUNT <num>    Number of lines to read

ARGS:
    <path>    The path to the file to read
```
