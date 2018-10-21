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

Since all the options are optional generating a single password is as easy as  <sup>1</sup>:
```
$ pgrs
0dKQf0XGXKIqTafu4$kc
```

A more involved example using all the options <sup>1</sup>:
```
$ pgrs pgrs -l 8 -c 4 -a ln
7vgqzoyt
l8grwjfu
fzdl0okn
cvkajh28
```

### Compiling

Since this is Rust application you'll need to go to https://www.rust-lang.org and install it. Then clone the repo and open it in terminal. Run:

 ```
 $ cargo build --release
 ```

You'll find the `pgrs` binary in `./target/release`.

**Notes**  
<sup>1</sup> *It should go without saying - but just in case - do not use any of
the passwords in this document. They are samples and obviously compromised.*
