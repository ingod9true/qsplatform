use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub workers: usize,
    pub keep_alive: u32,
    pub json_limit: u32,     //m
    pub form_limit: u32,     //m
    pub data_form: u32,      //m
    pub file: u32,           //m

    pub ctrl_c: bool,
}








#[test]
fn xx(){

}
