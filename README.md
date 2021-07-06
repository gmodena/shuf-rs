![](https://github.com/gmodena/shuf-rs/workflows/Build%20and%20test/badge.svg)

# shuf-rs

Shuffle lines of text with [reservior sampling](https://en.wikipedia.org/wiki/Reservoir_sampling).


# Build

```
cargo build --release
```

The resulting binary is found under `target/release/shuf`.

# Usage

```
USAGE:
    shuf [OPTIONS] [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --head-count=COUNT <num>    Number of lines to read

ARGS:
    <path>    The path to the file to read
```

# mod shuf

`src/shuf.rs` contains a reusable implementation of reservior sampling, that works on any iterable.
