use std::path::PathBuf;
use std::{env::current_dir, process};

use crate::finder::find_changelog;

pub fn find_from_cwd() -> Result<PathBuf, String> {
    let cwd = current_dir();
    if cwd.is_err() {
        return Result::Err(String::from(
            "Could not determine current working directory!",
        ));
    }

    let changelog = find_changelog(&cwd.unwrap());
    if changelog.is_none() {
        return Result::Err(String::from(
            "Could not find a changelog relative to the current working directory!",
        ));
    }

    return Result::Ok(changelog.unwrap());
}

pub fn find() {
    match find_from_cwd() {
        Result::Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
        Result::Ok(path) => {
            let changelog_path = path.to_str();
            if changelog_path.is_none() {
                eprintln!("Could not convert path to string!");
                process::exit(1);
            }

            println!("{}", changelog_path.unwrap());
        }
    }
}
