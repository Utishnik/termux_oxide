use smol::process::Command as smol_cmd;
use std::io;
use std::process::{Command, Output};
//use smol::process::unix::*;
use futures::{FutureExt, select};
use smol::Timer;
use smol::prelude::Future;

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

pub fn ret_cmd_arg_async_timeout(cmd: &str, arg: &str) -> impl Future<Output = io::Result<Output>> {
    smol_cmd::new(cmd).arg(arg).output()
}

fn ret_cmd_async_timeout(cmd: &str) -> impl Future<Output = io::Result<Output>> {
    smol_cmd::new(cmd).output()
}

fn ret_cmd_with_args_async_timeout(
    cmd: &str,
    args: &[&str],
) -> impl Future<Output = io::Result<Output>> {
    smol_cmd::new(cmd).args(args).output()
}

pub enum ResTimeOut {}
pub enum Res {
    ResTimeOut,
    Out(io::Result<Output>),
}

impl Res {
    pub fn unwrap(self) -> io::Result<Output> {
        match self {
            Self::ResTimeOut => {
                panic!("ResTimeOut is ResTimeOut");
            }
            Self::Out(x) => x,
        }
    }
}

async fn run_cmd_time_out(cmd: &str, arg: &str, delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_arg_async_timeout(cmd,arg).fuse() => {Res::Out(res)},
        time_out = timer.fuse() => {Res::ResTimeOut},
    }
}

async fn run_cmd_time_out_not_arg(cmd: &str, delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_async_timeout(cmd).fuse() => {Res::Out(res)},
        _time_out = timer.fuse() => {Res::ResTimeOut},
    }
}

async fn run_cmd_with_args_time_out(cmd: &str, args: &[&str], delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_with_args_async_timeout(cmd,args).fuse() => {Res::Out(res)},
        time_out = timer.fuse() => {Res::ResTimeOut},
    }
}

pub async fn async_run_api_cmd(cmd: &str) -> io::Result<String> {
    let output = smol_cmd::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn async_run_api_cmd_with_args(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = smol_cmd::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .args(args)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub enum TimeOutRes<T> {
    TimeOut,
    Ok(T),
}

pub async fn async_run_api_cmd_timeout(
    cmd: &str,
    delay: core::time::Duration,
) -> TimeOutRes<io::Result<String>> {
    let output: Res = run_cmd_time_out_not_arg(cmd, delay).await;
    if let Res::ResTimeOut = output {
        return TimeOutRes::TimeOut;
    }
    let unwrap: Result<Output, io::Error> = output.unwrap();
    /*
        .arg(cmd)
        .output()
        .await?;
    */
    if unwrap.is_err() {
        return TimeOutRes::Ok(Err(unwrap.unwrap_err()));
    }
    let valid: Output = unwrap.unwrap();
    TimeOutRes::Ok(Ok(String::from_utf8_lossy(&valid.stdout).to_string()))
}

pub async fn async_run_api_cmd_with_args_timeout(
    cmd: &str,
    args: &[&str],
    delay: core::time::Duration,
) -> TimeOutRes<io::Result<String>> {
    let output: Res = run_cmd_with_args_time_out(cmd, args, delay).await;
    if let Res::ResTimeOut = output {
        return TimeOutRes::TimeOut;
    }
    let unwrap: Result<Output, io::Error> = output.unwrap();
    /*
        .arg(cmd)
        .args(args)
        .output()
        .await?;
    */
    if unwrap.is_err() {
        return TimeOutRes::Ok(Err(unwrap.unwrap_err()));
    }
    let valid: Output = unwrap.unwrap();
    TimeOutRes::Ok(Ok(String::from_utf8_lossy(&valid.stdout).to_string()))
}
