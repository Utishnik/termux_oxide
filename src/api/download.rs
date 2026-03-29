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
    pub fn convert_args_to_str(&self) -> Box<[&str]> {
        let description: &str = &*self.description.as_str();
        let title: &str = &*self.title.as_str();
        let path: &str = &*self.path.as_str();
        let ret: Box<[&str]> = Box::new([description, title, path]);
        ret
    }
    pub async fn async_run(&self) -> Result<(), Error> {
        let args: Box<[&str]> = self.convert_args_to_str();
        let command: Result<String, Error> =
            async_run_api_cmd_with_args("termux-download", &*args).await;
        match command {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn async_run_timeout(&self, delay: core::time::Duration) -> Result<(), TermuxError> {
        let args: Box<[&str]> = self.convert_args_to_str();
        let command: TimeOutRes<Result<String, Error>> =
            async_run_api_cmd_with_args_timeout("termux-download", &*args, delay).await;
        match command {
            TimeOutRes::Ok(Ok(_)) => Ok(()),
            TimeOutRes::Ok(Err(e)) => Err(TermuxError::IOError(e)),
            TimeOutRes::TimeOut => Err(TermuxError::TimeOut),
        }
    }
}
