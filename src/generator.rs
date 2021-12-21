use rand::{self, Rng};

pub fn gen_password(num: u32, length: usize, inc_numbers: bool, inc_symbols: bool) -> Vec<String> {
    let mut rng = rand::thread_rng();

    let mut password_list = Vec::<String>::new();
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

    while num_of_generated < num {
        let mut current_password: Vec<String> = vec![];

        while current_password.len() < length {
            let mut random_char = alpha[rng.gen_range(0..alpha.len())].to_string();

            let is_uppercase = rng.gen_bool(0.5);

            if is_uppercase {
                random_char = random_char.to_uppercase();
            }

            current_password.push(random_char);
        }

        let assembled_password = current_password.iter().map(|x| x.to_string()).collect();

        password_list.push(assembled_password);

        num_of_generated += 1;
    }

    return password_list;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_password() {
        let passwords = gen_password(2, 12, false, false);

        assert_eq!(passwords.len(), 2);
        assert_eq!(passwords.first().unwrap().len(), 12);
    }
}
