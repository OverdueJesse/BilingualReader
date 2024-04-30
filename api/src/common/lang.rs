use core::fmt;
use std::str::FromStr;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Lang {
    EN,
    JP,
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(input: &str) -> Result<Lang, Self::Err> {
        match input. to_uppercase().as_str() {
            "EN"  => Ok(Lang::EN),
            "JP"  => Ok(Lang::JP),
            "ENGLISH" => Ok(Lang::EN),
            "日本語" => Ok(Lang::JP),
            _      => Err(()),
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::EN => write!(f, "EN"),
            Lang::JP => write!(f, "JP"),
        }
    }
}

impl <'r> FromParam<'r> for Lang {
    type Error = ();
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match Lang::from_str(&param) {
            Ok(lang) => Ok(lang),
            Err(e) => Err(e),
        }
    }
}