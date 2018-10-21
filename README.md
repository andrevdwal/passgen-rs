# Password Generator RS

*A simple CLI password generator written in Rust*

### Usage
```
$ pgrs -l <LENGTH> -c <COUNT> -a <ALPHABET>
```

- `-l, --length`: The length of the password, default 20.
- `-c, --count`: Number of passwords to generate, default 1.
- `-a, --alphabet`: "Alphabets" to use including:
  - `d`=default/all, equivalent of `lusn`
  - `l`=lowercase
  - `u`=uppercase
  - `s`=special characters
  - `n`=numbers

Since all the options are optional generating a single password is as easy as:
```
$ pgrs
0dKQf0XGXKIqTafu4$kc
```

A more involved example using all the options:
```
$ pgrs pgrs -l 8 -c 4 -a ln
7vgqzoyt
l8grwjfu
fzdl0okn
cvkajh28
```

### Compiling

 1. Since this is Rust application you'll need to go to https://www.rust-lang.org
    and install it.

 1. Checkout the repo and open it in terminal.

 1. ```$ cargo build --release```

You'll find the `pgrs` binary in `./target/release`
