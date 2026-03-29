use crate::api::run::async_run_api_cmd_timeout;

use super::errors::TermuxError;
use super::run::async_run_api_cmd;
use crate::api::run::TimeOutRes;
use serde::{Deserialize, Serialize};
use simd_json::{json, prelude::*, serde as simd_serde};
use smol::process::Command as smol_cmd;
use std::fs;

static JSON_START_SIZE: usize = 256;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Sms{
    threadid: usize,
    #[serde(rename = "type")]
    type_sms: String,
    read: bool,
    sender: String,
    address: String,
    number: String,
    received: String,
    body: String,
    #[serde(rename = "_id")]
    id: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct SmsList{
    inner: Vec<Sms>,
}