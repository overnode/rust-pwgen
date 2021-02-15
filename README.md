# rust-pwgen

Password generator made in Rust. Made mainly because I was courious about [Rust](https://www.rust-lang.org/), and had to make something. Besides you sure can't have enough password (or random gibblish) generators - cheers :beer:

Compile with

```
cargo build --release
```

Run with

```
./target/release/pwgen
```

```
USAGE:
pwgen [FLAGS] [OPTIONS] [ARGS]

FLAGS:
-h, --help Prints help information
-n Include numbers
-s Include symbols
-V, --version Prints version information

OPTIONS:
-d <delimiter> Delimiter for generated passwords

ARGS:
<NUM_PASSWORDS> Number of passwords to generate [default: 1]
<LENGTH> Length of passwords [default: 8]
```
