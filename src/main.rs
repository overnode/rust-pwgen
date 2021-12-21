use clap::{value_t, App, Arg};
mod generator;

fn main() {
    let matches = App::new("pwgen")
        .version("0.8")
        .author("Thomas G. Johansen <thomas.johansen@overnode.com>")
        .about("Generates passwords")
        .arg(
            Arg::with_name("NUM_PASSWORDS")
                .help("Number of passwords to generate")
                .default_value("1")
                .index(1),
        )
        .arg(
            Arg::with_name("LENGTH")
                .help("Length of passwords")
                .default_value("8")
                .index(2),
        )
        .arg(Arg::with_name("numbers").short("n").help("Include numbers"))
        .arg(Arg::with_name("symbols").short("s").help("Include symbols"))
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .takes_value(true)
                .help("Delimiter for generated passwords"),
        )
        .get_matches();

    let delimiter = value_t!(matches, "delimiter", char).unwrap_or('\n');

    let number_of_passwords = value_t!(matches, "NUM_PASSWORDS", u32).unwrap();
    let password_size = value_t!(matches, "LENGTH", usize).unwrap();

    let use_symbols = matches.is_present("symbols");
    let use_numbers = matches.is_present("numbers");

    let password_list =
        generator::gen_password(number_of_passwords, password_size, use_numbers, use_symbols);

    let mut counter = 1;
    for pw in password_list {
        print!("{}", pw);

        if counter < number_of_passwords {
            print!("{}", delimiter);
        } else {
            print!("\n");
        }
        counter += 1;
    }
}
