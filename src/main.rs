use clap::{value_t, App, Arg};
use rand::{self, Rng};

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

    let number_of_passwords: u32 = value_t!(matches, "NUM_PASSWORDS", u32).unwrap();
    let password_size = value_t!(matches, "LENGTH", usize).unwrap();

    let use_symbols = matches.is_present("symbols");
    let use_numbers = matches.is_present("numbers");

    let password_list = gen_password(
        &number_of_passwords,
        &password_size,
        use_numbers,
        use_symbols,
    );

    let mut counter = 1;
    for pw in password_list {
        print!("{}", &pw);

        if counter < number_of_passwords {
            print!("{}", &delimiter);
        } else {
            print!("\n");
        }
        counter += 1;
    }
}

fn gen_password(num: &u32, length: &usize, inc_numbers: bool, inc_symbols: bool) -> Vec<String> {
    let mut rng = rand::thread_rng();

    let mut password_list: Vec<String> = Vec::<String>::new();
    let mut num_of_generated = 0;

    let mut alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();
    let symbols: Vec<char> = "-_!#?%&()@£$".chars().collect();

    if inc_numbers {
        alpha.extend(numbers);
    }

    if inc_symbols {
        alpha.extend(symbols);
    }

    while &num_of_generated < num {
        let mut current_password: Vec<String> = vec![];

        while &current_password.len() < &length {
            let mut random_char = alpha[rng.gen_range(0..alpha.len())].to_string();

            let is_uppercase = rng.gen_bool(0.5);

            if is_uppercase {
                random_char = random_char.to_uppercase();
            }

            current_password.push(random_char);
        }

        let p: String = current_password.iter().map(|x| x.to_string()).collect();

        password_list.push(p);

        num_of_generated += 1;
    }

    return password_list;
}
