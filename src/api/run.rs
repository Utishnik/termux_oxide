use std::io;
use std::process::Command;

pub fn run_api_cmd(cmd: &str) -> io::Result<String> {
    let output = Command::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_api_cmd_with_args(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .args(args)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn run_cmd(cmd: &str, arg: &str) -> io::Result<()> {
    let _output = Command::new(cmd).arg(arg).output()?;
    Ok(())
}

pub fn run_cmd_with_args(cmd: &str, args: &[&str]) -> io::Result<()> {
    let _output = Command::new(cmd).args(args).output()?;
    Ok(())
}
