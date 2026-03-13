use std::{io::{BufRead, BufReader}, process::{Command, Stdio}, sync::mpsc::Sender, thread};

use crate::{config::NIXOS_CONFIGURATION_PATH, ui::components::organisms::TerminalLine};

pub fn upgrade(tx: Sender<TerminalLine>) {
    let mut git_pull_cmd = Command::new("git");
    git_pull_cmd.arg("pull")
        .current_dir(NIXOS_CONFIGURATION_PATH)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    run_and_stream(git_pull_cmd, &tx);

    let mut rebuild_cmd = Command::new("pkexec");
    rebuild_cmd.arg("nixos-rebuild")
        .arg("switch")
        .current_dir(NIXOS_CONFIGURATION_PATH)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    run_and_stream(rebuild_cmd, &tx);
}

fn run_and_stream(mut cmd: Command, tx: &Sender<TerminalLine>) {
    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child.stdout.take().expect("missing stdout");
    let stderr = child.stderr.take().expect("missing stderr");

    let tx_out = tx.clone();
    let out_handle = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line.unwrap_or_default();
            if tx_out.send(TerminalLine::Normal(line)).is_err() {
                break;
            }
        }
    });

    let tx_err = tx.clone();
    let err_handle = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line.unwrap_or_default();
            if tx_err.send(TerminalLine::Error(line)).is_err() {
                break;
            }
        }
    });

    let _ = out_handle.join();
    let _ = err_handle.join();
    let _ = child.wait();
}
