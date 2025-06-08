use std::io::{self, Read, Write};
use crossterm::terminal;
use tokio::task;

pub fn read_password(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let raw_mode_enabled = terminal::enable_raw_mode().is_ok();
    if !raw_mode_enabled {
        eprintln!("Warning: Password will be displayed in plain text");
    }

    let mut buffer = Vec::new();
    loop {
        let mut byte = [0];
        if io::stdin().read(&mut byte).is_err() || byte[0] == b'\n' || byte[0] == b'\r' {
            break;
        }

        match byte[0] {
            8 | 127 => {
                if buffer.pop().is_some() {
                    print!("\x08 \x08");
                    io::stdout().flush().unwrap();
                }
            }
            3 => {
                println!();
                return None;
            }
            _ => {
                buffer.push(byte[0]);
                print!("*");
                io::stdout().flush().unwrap();
            }
        }
    }

    if raw_mode_enabled {
        terminal::disable_raw_mode().unwrap();
    }
    println!();

    Some(String::from_utf8_lossy(&buffer).into_owned())
}

pub fn read_password_and_confirm(
    prompt: &str,
    confirm_prompt: &str
) -> Option<String> {
    loop {
        let pw1 = read_password(prompt).unwrap();
        let pw2 = read_password(confirm_prompt).unwrap();

        if pw1 == pw2 {
            return Some(pw1);
        }
        eprintln!("The passwords entered twice do not match, please try again.");
    }
}

pub fn confirm(prompt: &str) -> bool {
    let prompt = format!("{} [Y/n]: ", prompt);
    loop {
        let input = read_password(&prompt).unwrap_or_default();
        match input.to_lowercase().as_str() {
            "y" | "yes" | "" => return true,
            "n" | "no" => return false,
            _ => eprintln!("Invalid input, please enter Y or n."),
        }
    }
}

pub async fn read_password_async(prompt: &str) -> Option<String> {
    let prompt = prompt.to_owned();
    task::spawn_blocking(move || read_password(&prompt))
        .await
        .unwrap_or_else(|_| None)
}

pub async fn read_password_and_confirm_async(
    prompt: &str,
    confirm_prompt: &str
) -> Option<String> {
    loop {
        let pw1 = read_password_async(prompt).await.unwrap();
        let pw2 = read_password_async(confirm_prompt).await.unwrap();

        if pw1 == pw2 {
            return Some(pw1);
        }
        eprintln!("The passwords entered twice do not match, please try again.");
    }
}

pub async fn confirm_async(prompt: &str) -> bool {
    let prompt = format!("{} [Y/n]: ", prompt);
    loop {
        let input = read_password_async(&prompt).await.unwrap_or_default();
        match input.to_lowercase().as_str() {
            "y" | "yes" | "" => return true,
            "n" | "no" => return false,
            _ => eprintln!("Invalid input, please enter Y or n."),
        }
    }
}