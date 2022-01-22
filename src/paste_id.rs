use std::fmt;
use std::borrow::Cow;
//clone on write -> Return &str or string 

use rand::{self, Rng};
use rocket::request::FromParam;
use rocket::http::RawStr;

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
// ' is reference
pub struct PasteId<'a>(Cow<'a,str>);

fn valid_id(id: &str) -> bool{
    id.chars().all(|c| {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c<= 'Z') || ( c >= '0' && c <= '9' )
    })
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<PasteId<'a>, &'a RawStr> {
        match valid_id(param) {
            true => Ok(PasteId(Cow::Borrowed(param))),
            false => Err(param)
        }
    }
}

impl<'a> PasteId<'a> {
    pub fn new(size: usize) -> PasteId<'static>{
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size{
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteId(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for PasteId<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}