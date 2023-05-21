use std::process;

use crate::finder::find_changelog_relative_to_cwd;

pub fn find() {
    match find_changelog_relative_to_cwd() {
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
