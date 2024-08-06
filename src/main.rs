use rand::Rng;
use std::io;
use std::time::Instant;

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

fn is_valid_email(email: &str) -> bool {
    let at_position = email.find('@');
    let dot_position = email.rfind('.');

    if let (Some(at), Some(dot)) = (at_position, dot_position) {
        // Check if the "@" is before the last "."
        return at < dot && at > 0 && dot < email.len() - 1;
    }
    false
}

fn main() {
    // ANSI escape codes
    let orange = "\x1b[38;2;255;165;0m";
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    let email = loop {
        println!("{}[fastword]{} Enter your email address:", orange, reset);

        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Failed to read input");

        let email = email.trim();

        // bro is this even an email?
        if email.is_empty() || is_valid_email(email) {
            break email.to_string();
        } else {
            println!("Invalid email address. Please try again.");
        }
    };

    println!("{}[fastword]{} Scribe desired length...", orange, reset);
    println!("{}[fastword]{} Default is 12", orange, reset);

    // check length
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    let length: usize = if input.is_empty() {
        12
    } else {
        input.parse().expect("Please enter a valid number")
    };

    let start = Instant::now();

    let password = generate_password(length);

    let duration = start.elapsed();
    let ms = duration.as_secs_f64() * 1000.0;

    let time_color = if ms > 10.0 { red } else { green };

    println!("******************************");

    if !email.is_empty() {
        println!("email: {}", email);
    }

    println!("password: {}", password);
    println!("******************************");
    println!("{}in {:.2} ms using fastword{}", time_color, ms, reset);

    // Wait for user input before exiting (prevents window from closing immediately on Windows)
    println!("{}[fastword]{} exit when ready...", orange, reset);
    let mut exit_input = String::new();
    io::stdin()
        .read_line(&mut exit_input)
        .expect("Failed to read input");
}
