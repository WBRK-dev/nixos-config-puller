use std::process::Command;

use crate::{config::NIXOS_CONFIGURATION_PATH, ui::components::organisms::TerminalLine};

pub enum CheckForUpdatesResult {
    UpdateAvailable(Vec<TerminalLine>),
    NoUpdateAvailable,
}

pub fn check_for_updates() -> CheckForUpdatesResult {
    let fetch_status = Command::new("git")
        .arg("fetch")
        .current_dir(NIXOS_CONFIGURATION_PATH)
        .status();

    if fetch_status.map_or(true, |s| !s.success()) {
        return CheckForUpdatesResult::NoUpdateAvailable;
    }

    let rev_list_output = Command::new("git")
        .arg("rev-list")
        .arg("HEAD..@{u}")
        .current_dir(NIXOS_CONFIGURATION_PATH)
        .output();

    let rev_list_output = match rev_list_output {
        Ok(output) if output.status.success() => output,
        _ => return CheckForUpdatesResult::NoUpdateAvailable,
    };

    if rev_list_output.stdout.is_empty() {
        return CheckForUpdatesResult::NoUpdateAvailable;
    }

    let diff_output = Command::new("git")
        .arg("diff")
        .arg("--name-status")
        .arg("HEAD..@{u}")
        .current_dir(NIXOS_CONFIGURATION_PATH)
        .output();

    match diff_output {
        Ok(output) if output.status.success() => {
            let changed_files = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if changed_files.is_empty() {
                CheckForUpdatesResult::NoUpdateAvailable
            } else {
                CheckForUpdatesResult::UpdateAvailable(changed_files.split("\n").map(|line| TerminalLine::Normal(line.to_string())).collect())
            }
        }
        _ => CheckForUpdatesResult::NoUpdateAvailable,
    }
}
