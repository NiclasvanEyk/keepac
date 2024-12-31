use std::{
    env, io,
    path::Path,
    process::{Child, Command},
};

use crate::{errors::ErrorExitCode, KeepacCliError, SubcommandResult};

pub fn edit(path: &Path) -> SubcommandResult {
    let Some(changelog_path) = keepac::find::nearest_changelog_path(path) else {
        return Err(KeepacCliError {
            message: String::from("Failed to find CHANGELOG.md"),
            exit_code: ErrorExitCode::ChangelogNotFound,
        });
    };

    let mut process = try_terminal_editor(&changelog_path)
        .unwrap_or_else(|| try_system_editor(&changelog_path))?;
    let exit_status = process.wait()?;

    // TODO: exit_status

    Ok(())
}

pub fn try_terminal_editor(path: &Path) -> Option<io::Result<Child>> {
    Some(
        Command::new(env::var_os("EDITOR")?)
            .arg(path.to_str().unwrap())
            .spawn(),
    )
}

pub fn try_system_editor(path: &Path) -> io::Result<Child> {
    let path_str = path.to_str().unwrap();

    match env::consts::OS {
        "windows" => Command::new("cmd").args(["/c", "start", path_str]).spawn(),
        "macos" => Command::new("open").arg(path_str).spawn(),
        _ => Command::new("xdg-open").arg(path_str).spawn(),
    }
}
