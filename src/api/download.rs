use crate::api::run::async_run_api_cmd_timeout;
use super::errors::TermuxError;
use super::run::async_run_api_cmd;
use crate::api::run::TimeOutRes;
use smol::process::Command as smol_cmd;
use std::io::Error;

pub struct Download;

impl Download {
    pub async fn async_run() -> Result<(),Option<Error>>{
        let mut command: smol_cmd = smol_cmd::new("termux-contact-list");
        let output: Result<std::process::Output, std::io::Error> = command.output().await;
        match output {
                Ok(output) => {
                    if output.status.success() {
                        return Ok(());
                    }
                    Err(None)
                }
                Err(e) => Err(Some(e)),
        }
    }

    pub async fn async_run_timeout(delay: core::time::Duration) -> {
        let command: super::run::TimeOutRes<Result<String, std::io::Error>> =
            async_run_api_cmd_timeout("termux-contact-list", delay).await;
        return match command {
            TimeOutRes::Ok(Ok(mut output)) => unsafe {
                let s: &mut str = std::str::from_utf8_mut(output.as_bytes_mut()).unwrap();
                let list: ContactList = simd_json::serde::from_str::<ContactList>(s).unwrap();
                Ok(list)
            },
            TimeOutRes::Ok(Err(e)) => Err(TermuxError::IOError(e)),
            TimeOutRes::TimeOut => Err(TermuxError::TimeOut),
        };
    }
}