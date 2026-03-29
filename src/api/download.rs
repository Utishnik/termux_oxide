use super::errors::TermuxError;
use crate::api::run::TimeOutRes;
use crate::api::run::{async_run_api_cmd_with_args, async_run_api_cmd_with_args_timeout};
use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Download {
    pub description: String,
    pub title: String,
    pub path: String,
}

impl Download {
    pub async fn async_run(args: &[&str]) -> Result<(), Error> {
        let command: Result<String, Error> =
            async_run_api_cmd_with_args("termux-download", args).await;
        match command {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn async_run_timeout(
        args: &[&str],
        delay: core::time::Duration,
    ) -> Result<(), TermuxError> {
        let command: TimeOutRes<Result<String, Error>> =
            async_run_api_cmd_with_args_timeout("termux-download", args, delay).await;
        match command {
            TimeOutRes::Ok(Ok(_)) => Ok(()),
            TimeOutRes::Ok(Err(e)) => Err(TermuxError::IOError(e)),
            TimeOutRes::TimeOut => Err(TermuxError::TimeOut),
        }
    }
}
