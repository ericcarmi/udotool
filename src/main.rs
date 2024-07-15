use clap::Parser;
use commands::Commands;
use enigo::{Direction, Enigo, Keyboard, Settings};
use std::time::Duration;
mod commands;

/// Simple program to simulate writing text
/// example string
/// ("%p "+y) ctrl[down] (%run ) ctrl-v enter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// string of commands to be parsed into typing and action keys
    #[arg(short, long)]
    text: String,
}

fn main() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let args = Args::parse();
    println!("{:?}", args);
    let mut command = "".to_string();
    let mut is_text = false;
    let mut text = "".to_string();

    let mut keys = vec![];

    let text_space = args.text.to_lowercase() + " ";

    for char in text_space.chars() {
        if char == ')' {
            is_text = false;
            write_text(text.clone(), &mut enigo);
            // println!("{:?}", text);

            text = "".to_string();
            continue;
        }
        if is_text {
            text += char.to_string().as_str();
            continue;
        }
        // text is surrounded by parentheses
        if char == '(' {
            // read until right parentheses, if none is reached, return error
            is_text = true;
        }
        // commands are separated by spaces
        else if char == ' ' && !command.is_empty() {
            let spl = command.split('-');
            if spl.clone().count() == 1 {
                if let Some(c) = Commands.get(command.as_str()) {
                    // println!("{:?}", c);
                    // keys.push(c);
                    command = "".to_string();
                } else {
                    return Err("command not recognized, sorry can't continue".to_string());
                }
            } else {
                // want to skip last, so reverse
                for s in spl.clone().rev().skip(1) {
                    if let Some(key) = Commands.get(s) {
                        keys.push(key);
                    } else {
                        return Err("Key not recognized".to_string());
                    }
                }
                if let Some(last) = spl.last() {
                    if let Some(last_cmd) = Commands.get(last) {
                        // keys are held down together, then "last" is pressed, then keys are released
                        // println!("{:?}", keys);
                        for key in keys.clone() {
                            enigo.key(*key, Direction::Press);
                            std::thread::sleep(Duration::from_micros(100));
                        }
                        enigo.key(*last_cmd, Direction::Click);
                        for key in keys {
                            enigo.key(*key, Direction::Release);
                            std::thread::sleep(Duration::from_micros(100));
                        }

                        keys = vec![];
                        command = "".to_string();
                    } else {
                        return Err("failed to get command for last part".to_string());
                    }
                } else {
                    return Err("failed to get last part of slice".to_string());
                }
            }
        } else if char != ' ' {
            command += char.to_string().as_str();
        }
    }
    // last command processed here with no space...or add a space before so you don't have to do this twice
    Ok(())
}

fn write_text(text: String, enigo: &mut Enigo) {
    for t in text.chars() {
        let _r = enigo.text(t.to_string().as_str());
    }
    // apparently it needs a delay between writing text and key presses
    std::thread::sleep(Duration::from_millis(10));
}
