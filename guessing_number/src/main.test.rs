#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guessing_number() {
        // Test a correct guess
        let mut input = "500\n".as_bytes();
        let mut output = Vec::new();
        let secret = rand::thread_rng().gen_range(1, 1001);
        let expected_output = format!("Adivinha um número\nPor favor, Digite seu palpite de um número\nO número inserido é: {}\nVocê ganhou!\n", secret);
        io::stdin().read_exact(&mut input).unwrap();
        main_with_input(&mut input, &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);

        // Test a guess that is too high
        input = "1000\n".as_bytes();
        output = Vec::new();
        let expected_output = format!("Adivinha um número\nPor favor, Digite seu palpite de um número\nO número inserido é: {}\nMuito abaixo\n", secret);
        io::stdin().read_exact(&mut input).unwrap();
        main_with_input(&mut input, &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);

        // Test a guess that is too low
        input = "1\n".as_bytes();
        output = Vec::new();
        let expected_output = format!("Adivinha um número\nPor favor, Digite seu palpite de um número\nO número inserido é: {}\nMuito acima\n", secret);
        io::stdin().read_exact(&mut input).unwrap();
        main_with_input(&mut input, &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);

        // Test an invalid guess
        input = "not a number\n".as_bytes();
        output = Vec::new();
        let expected_output =
            format!("Adivinha um número\nPor favor, Digite seu palpite de um número\n");
        io::stdin().read_exact(&mut input).unwrap();
        main_with_input(&mut input, &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }

    fn main_with_input(input: &mut dyn io::Read, output: &mut dyn io::Write) {
        let secret = rand::thread_rng().gen_range(1, 1001);
        let mut input = io::BufReader::new(input);
        let mut output = io::BufWriter::new(output);
        writeln!(output, "Adivinha um número").unwrap();

        loop {
            writeln!(output, "Por favor, Digite seu palpite de um número").unwrap();

            let mut guess = String::new();
            input.read_line(&mut guess).unwrap();
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            writeln!(output, "O número inserido é: {}", guess).unwrap();
            match guess.cmp(&secret) {
                Ordering::Equal => {
                    writeln!(output, "Você ganhou!").unwrap();
                    break;
                }
                Ordering::Less => writeln!(output, "Muito abaixo").unwrap(),
                Ordering::Greater => writeln!(output, "Muito acima").unwrap(),
            }
        }
    }
}
