extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

pub struct Data {
    pub quantity: u64;
}

pub fn read_csv() -> Result<Vec<Data>, Box<Error>> {
    let file = File::open()
}