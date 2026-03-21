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
pub struct Contact {
    name: String,
    number: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ContactList {
    inner: Vec<Contact>,
}

impl Default for ContactList {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactList {
    pub fn new() -> Self {
        ContactList { inner: Vec::new() }
    }
    pub fn new_with_size(cap: usize) -> Self {
        ContactList {
            inner: Vec::with_capacity(cap),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SoAContactList {
    name: Vec<String>,
    number: Vec<String>,
}

impl ContactList {
    pub fn add(&mut self, item: Contact) {
        self.inner.push(item);
    }
}

//TODO https://github.com/Utishnik/json-fmt/tree/master
fn format_json_bytes_with_indent(json_bytes: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(JSON_START_SIZE);
    let mut indent_level: usize = 0;
    let indent: &[u8; 2] = b"  "; //todo small/tiny vec

    for byte in &json_bytes {
        match byte {
            b'{' | b'[' => {
                result.push(*byte);
                result.push(b'\n');
                indent_level += 1;
                result.extend_from_slice(&indent.repeat(indent_level));
            }
            b'}' | b']' => {
                result.push(b'\n');
                indent_level -= 1;
                result.extend_from_slice(&indent.repeat(indent_level));
                result.push(*byte);
            }
            b',' => {
                result.push(*byte);
                result.push(b'\n');
                result.extend_from_slice(&indent.repeat(indent_level));
            }
            _ => {
                result.push(*byte);
            }
        }
    }

    result
}

pub fn ser_list_json(list: ContactList, path_file: String) {
    let mut buf: Vec<u8> = Vec::with_capacity(JSON_START_SIZE);
    let write_res: Result<(), simd_json::Error> = simd_serde::to_writer(&mut buf, &list);
    if write_res.is_err() {
        println!("ser_list_json ОШИБКА серелизации");
        return;
    }
    let format_buf: Vec<u8> = format_json_bytes_with_indent(buf);
    let res: Result<(), std::io::Error> = fs::write(path_file, format_buf);
    if res.is_err() {
        println!("ОШИБКА сохранения");
    }
}

pub fn file_read(path: String) -> std::io::Result<Vec<u8>> {
    let bytes: Vec<u8> = fs::read(path)?;
    Ok(bytes)
}

pub fn de_list_json(cfg_path: String) -> Result<ContactList, ()> {
    let res: Result<Vec<u8>, std::io::Error> = file_read(cfg_path);
    if res.is_err() {
        println!("ОШИБКА чтения");
        return Err(());
    }
    let unwrap_res: Result<String, std::string::FromUtf8Error> = String::from_utf8(res.unwrap());
    if unwrap_res.is_err() {
        println!("Невалидный UTF-8: {:0}", unwrap_res.unwrap_err());
        return Err(());
    }
    let mut res_str: String = unwrap_res.unwrap();
    unsafe {
        let des: Result<ContactList, simd_json::Error> =
            simd_json::serde::from_str::<ContactList>(&mut res_str);
        if des.is_err() {
            println!("ОШИБКА десерилизации: {:0}", des.unwrap_err());
            return Err(());
        }
        Ok(des.unwrap())
    }
}

impl ContactList {
    pub async fn async_run() -> Result<Self, TermuxError> {
        let mut command = smol_cmd::new("termux-contact-list");
        let output: Result<std::process::Output, std::io::Error> = command.output().await;
        match output {
            Ok(mut output) => {
                if output.status.success() {
                    unsafe {
                        let s: &mut str = std::str::from_utf8_mut(&mut output.stdout).unwrap();
                        let list: ContactList =
                            simd_json::serde::from_str::<ContactList>(s).unwrap();

                        return Ok(list);
                    }
                }
                Err(TermuxError::Output(output.to_owned()))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
    pub async fn async_run_timeout(delay: core::time::Duration) -> Result<Self, TermuxError> {
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

#[test]
fn test_ser() {
    let mut test_list: ContactList = ContactList::new_with_size(10);
    test_list.add(Contact {
        name: "zxc".to_string(),
        number: "1488".to_string(),
    });
    test_list.add(Contact {
        name: "zxc2".to_string(),
        number: "1337".to_string(),
    });
    test_list.add(Contact {
        name: "aaa".to_string(),
        number: "1111".to_string(),
    });
    test_list.add(Contact {
        name: "u".to_string(),
        number: "1".to_string(),
    });
    ser_list_json(test_list, "test.json".to_string());
}

#[test]
fn test() {
    smol::block_on(async {
        let test_list: ContactList = ContactList::async_run().await.unwrap();
        println!("{:?}", test_list);
    });
}
