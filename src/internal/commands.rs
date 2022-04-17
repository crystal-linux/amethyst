use crate::error::{AppError, AppResult};
use crate::internal::uwu_enabled;
use crate::uwu;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader};
use std::process::{ChildStderr, ChildStdout, Command, Stdio};

/// Executes a makepkg command
#[inline]
pub fn makepkg<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    run_command("makepkg", args)
}

/// Executes a git command
#[inline]
pub fn git<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    run_command("git", args)
}

/// Executes a bash command
#[inline]
pub fn bash<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    run_command("bash", args)
}

/// Runs pacman with sudo
pub fn sudo_pacman<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    let mut pacman_args = args
        .into_iter()
        .map(|i: S| OsString::from(i.as_ref()))
        .collect::<Vec<OsString>>();
    let mut sudo_args = vec![OsString::from("pacman")];
    sudo_args.append(&mut pacman_args);
    sudo(sudo_args)
}

/// Executes a pacman command
#[inline]
pub fn pacman<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    run_command("pacman", args)
}

#[inline]
pub fn sudo<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> AppResult<String> {
    run_command("sudo", args)
}

/// Runs a command and parses its output as string
fn run_command<S1: AsRef<OsStr>, I: IntoIterator<Item = S2>, S2: AsRef<OsStr>>(
    command: S1,
    args: I,
) -> AppResult<String> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.as_mut().unwrap();
    let stderr = child.stderr.as_mut().unwrap();
    let stdout_str = read_stdout(stdout)?;
    let stderr_str = read_stderr(stderr)?;

    let status = child.wait()?;
    if status.success() {
        Ok(stdout_str)
    } else {
        Err(AppError::from(stderr_str))
    }
}

fn read_stdout(stdout: &mut ChildStdout) -> AppResult<String> {
    let mut stdout_str = String::new();
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        let line = line?;
        if uwu_enabled() {
            println!("{}", uwu!(&*line))
        } else {
            println!("{}", &line);
        }
        stdout_str.push_str(&line);
        stdout_str.push_str("\n");
    }

    Ok(stdout_str)
}

fn read_stderr(stderr: &mut ChildStderr) -> AppResult<String> {
    let mut stderr_str = String::new();
    let stderr_reader = BufReader::new(stderr);

    for line in stderr_reader.lines() {
        let line = line?;
        if uwu_enabled() {
            eprintln!("{}", uwu!(&line))
        } else {
            eprintln!("{}", &line);
        }
        stderr_str.push_str(&line);
        stderr_str.push_str("\n");
    }

    Ok(stderr_str)
}
