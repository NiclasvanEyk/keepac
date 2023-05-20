use std::{env::current_dir, process};

use crate::finder::find_changelog;

pub fn find() {
    let cwd = current_dir();
    if cwd.is_err() {
        eprintln!("Could not determine current working directory!");
        process::exit(1);
    }

    let changelog = find_changelog(&cwd.unwrap());
    if changelog.is_none() {
        eprintln!("Could not find a changelog relative to the current working directory!");
        process::exit(1);
    }

    let temp = changelog.unwrap();
    let changelog_path = temp.to_str();
    if changelog_path.is_none() {
        eprintln!("Could not convert path to string!");
        process::exit(1);
    }

    println!("{}", changelog_path.unwrap());
}
