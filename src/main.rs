use std::io;

fn main() {
    loop {
        println!("Write 1 to calculate fr_to_cel or 2 to calculate cel_to_fr.");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Invalid input");

        let current_number: u8 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Write your integer value.");

        let mut second_line = String::new();

        io::stdin()
            .read_line(&mut second_line)
            .expect("Invalid input");

        let value: f32 = match second_line.trim().parse() {
            Ok(value) => value,
            Err(_) => continue
        };

        if current_number == 1 {
            println!("{}", fr_to_cel(value));
        } else if current_number == 2 {
            println!("{}", cel_to_fr(value));
        } else {
            println!("Write 1 or 2 integer value.");
            continue;
        }
    }
}

fn fr_to_cel(mut number: f32) -> f32 {
    number = (number - 32.) / 1.8;
    number
}

fn cel_to_fr(mut number: f32) -> f32 {
    number = (number * 1.8) + 32.;
    number
}