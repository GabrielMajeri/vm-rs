use std::{io, result};

pub type Error = io::Error;
pub type Result = result::Result<u32, Error>;
