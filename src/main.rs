#[macro_use]
extern crate clap;
extern crate rand;

use rand::prelude::*;

/// Default: -l 20 -c 1 -a d|lusn
fn main() {
    // arguments
    let cli_yaml = load_yaml!("cli.yml");
    let cli_args = clap::App::from_yaml(cli_yaml).get_matches();

    let arg_length: u8 = cli_args.value_of("length").unwrap_or("20").parse().unwrap();
    let arg_count: u16 = cli_args.value_of("count").unwrap_or("1").parse().unwrap();
    let arg_alphabet: &str = cli_args.value_of("alphabet").unwrap_or("d");

    // determine alphabets to use
    let alph_use_default: bool = arg_alphabet.contains("d");
    let alph_use_lower: bool = alph_use_default || arg_alphabet.contains("l");
    let alph_use_upper: bool = alph_use_default || arg_alphabet.contains("u");
    let alph_use_special: bool = alph_use_default || arg_alphabet.contains("s");
    let alph_use_numbers: bool = alph_use_default || arg_alphabet.contains("n");

    // generate passwords
    let alphabet: String = get_alphabet(alph_use_lower, alph_use_upper, alph_use_special, alph_use_numbers);
    for _ in 0..arg_count {
        let password = generate_password(&alphabet, arg_length);
        println!("{}", password);
    }
}

/// Buids an alphabet to use during password generation
fn get_alphabet(lower: bool, upper: bool, special: bool, numbers: bool) -> String {

    let mut alphabet = String::new();
    if lower {
        alphabet.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if upper {
        alphabet.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if special {
        alphabet.push_str("!@#$%&*?");
    }
    if numbers {
        alphabet.push_str("1234567890");
    }

    return alphabet;
}

/// Generates a password using randmom characters forom a provided alphabet
fn generate_password(alphabet: &String, length: u8) -> String {

    let mut password = String::new();
    let upper_idx = alphabet.len();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let idx = rng.gen_range(0, upper_idx);
        password.push(alphabet.chars().nth(idx).unwrap());
    }

    return password;
}
