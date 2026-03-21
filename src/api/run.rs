use smol::process::Command as smol_cmd;
use std::io;
use std::process::{Command, Output};
//use smol::process::unix::*;
use futures::{select, FutureExt};
use smol::prelude::Future;
use smol::Timer;

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
    let fut = smol_cmd::new(cmd).arg(arg).output();
    fut
}

pub fn ret_cmd_async_timeout(cmd: &str) -> impl Future<Output = io::Result<Output>> {
    let fut = smol_cmd::new(cmd).output();
    fut
}

pub fn ret_cmd_with_args_async_timeout(
    cmd: &str,
    args: &[&str],
) -> impl Future<Output = io::Result<Output>> {
    let fut = smol_cmd::new(cmd).args(args).output();
    fut
}

pub enum ResTimeOut {}
pub enum Res {
    ResTimeOut,
    Out(io::Result<Output>),
}
async fn run_cmd_time_out(cmd: &str, arg: &str, delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_arg_async_timeout(cmd,arg).fuse() => {return Res::Out(res);},
        time_out = timer.fuse() => {return Res::ResTimeOut;},
    }
}

async fn run_cmd_time_out_not_arg(cmd: &str, delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_async_timeout(cmd).fuse() => {return Res::Out(res);},
        _time_out = timer.fuse() => {return Res::ResTimeOut;},
    }
}

async fn run_cmd_with_args_time_out(cmd: &str, args: &[&str], delay: core::time::Duration) -> Res {
    let timer: Timer = Timer::after(delay);
    select! {
        res = ret_cmd_with_args_async_timeout(cmd,args).fuse() => {return Res::Out(res);},
        time_out = timer.fuse() => {return Res::ResTimeOut;},
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

pub async fn async_run_api_cmd_timeout(
    cmd: &str,
    delay: core::time::Duration,
) -> io::Result<String> {
    let output: Res =
        run_cmd_time_out_not_arg("/data/data/com.termux/files/usr/libexec/termux-api", delay).await;
    if let Res::ResTimeOut = output {}
    /*
        .arg(cmd)
        .output()
        .await?;
    */
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn async_run_api_cmd_with_args_tiemout(
    cmd: &str,
    args: &[&str],
    delay: core::time::Duration,
) -> io::Result<String> {
    let output = smol_cmd::new("/data/data/com.termux/files/usr/libexec/termux-api")
        .arg(cmd)
        .args(args)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
