// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// seems to be ok without using line above, and if it is used, help does not get printed
use clap::Parser;
use commands::Commands;
use enigo::{Direction, Enigo, Keyboard, Settings};
use std::time::Duration;
mod commands;

/// --
/// simple program to write text and press keys
/// -- it accepts one long argument separated by spaces
/// -- to write most strings, surround by [] -- this avoids issues with shells and quotes
/// -- to write the character ", use the command "quo" (with no quotes around it)
/// -- other commands represent key presses that do not write text
/// -- e.g. ctrl-shift-down, ctrl-v
/// -- example usage
/// win-dotool -t quo [%p] quo [+yu] ctrl-down [%run ] ctrl-v enter
#[derive(Parser, Debug)]
#[command(version, arg_required_else_help = true)]
struct Args {
    /// string of commands to be parsed into typing and action keys
    #[arg(short, long,  num_args = 1.., value_delimiter=' ')]
    text: Vec<String>,
}

fn main() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let args = Args::parse();
    let mut command = "".to_string();
    let mut is_text = false;
    let mut text = "".to_string();

    let trailing: String = format!("{:?}", args.text.join(" ").replace("\\", ""));

    let mut keys = vec![];

    // let text_space = args.text.unwrap().to_lowercase() + " ";
    let text_space = trailing.to_lowercase() + " ";

    for char in text_space.chars() {
        if char == '\\' {
            continue;
        }
        if char == ']' {
            is_text = false;
            write_text(text.clone(), &mut enigo);

            text = "".to_string();
            continue;
        }
        if is_text {
            text += char.to_string().as_str();
            continue;
        }
        // text is surrounded by parentheses
        if char == '[' {
            // read until right parentheses, if none is reached, return error
            is_text = true;
        }
        // commands are separated by spaces
        else if char == ' ' && !command.is_empty() {
            let spl = command.split('-');
            if spl.clone().count() == 1 {
                if let Some(c) = Commands.get(command.as_str()) {
                    enigo.key(*c, Direction::Click);
                    command = "".to_string();
                } else {
                    if command == "quo".to_string() {
                        write_text("\"".to_string(), &mut enigo);
                    }
                    command = "".to_string();
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
                        for key in keys.clone() {
                            enigo.key(*key, Direction::Press);
                            std::thread::sleep(Duration::from_micros(300));
                        }
                        enigo.key(*last_cmd, Direction::Click);
                        for key in keys {
                            enigo.key(*key, Direction::Release);
                            std::thread::sleep(Duration::from_micros(300));
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
        } else if char != ' ' && char != '\\' && char != '\"' {
            command += char.to_string().as_str();
        }
    }
    // last command processed here with no space...or add a space before so you don't have to do this twice
    Ok(())
}

fn write_text(text: String, enigo: &mut Enigo) {
    for t in text.chars() {
        let _r = enigo.text(t.to_string().as_str());
        std::thread::sleep(Duration::from_millis(1));
    }
    // apparently it needs a delay between writing text and key presses
    std::thread::sleep(Duration::from_millis(10));
}
