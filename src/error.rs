#![allow(unused_variables)]
#![allow(dead_code)]

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TinyUIError {
    General(String),
}

impl fmt::Display for TinyUIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for TinyUIError {
    fn description(&self) -> &str {
        match *self {
            TinyUIError::General(ref e) => e,
        }
    }
}
