use std::fs::OpenOptions;
use std::io::{Write, Read, Error};
use std::str::FromStr;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "backlight", about = "Set backlight")]
enum Options {
    Increase {
        value: u16,
    },
    Decrease {
        value: u16,
    },
}

fn set(mut value: u16, operation: &str) -> Result<(), Error> {
    let path = "/sys/class/backlight/intel_backlight/brightness";

    let mut file = OpenOptions::new().read(true).write(true).create(false).open(path)?;
    let mut current_value = String::new();
    file.read_to_string(&mut current_value).unwrap();

    let converted: u16 = u16::from_str(&current_value.trim()).unwrap();

    match operation {
        "increase" => {
            let result = (converted + value).to_string();
            file.write_all(&result.into_bytes()).unwrap();
        },
        "decrease" => {
            if value > converted {
                value = converted;
            }
            let result = (converted - value).to_string();
            file.write_all(&result.into_bytes()).unwrap(); 
        },
        &_ => ()
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    match Options::from_args() {
        Options::Increase{value} => {
            set(value, "increase")?;
        },
        Options::Decrease{value} => {
            set(value, "decrease")?;
        }
    }
    Ok(())
}
