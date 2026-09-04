use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn read_input(path: Option<&Path>) -> io::Result<(String, String)> {
    match path {
        Some(p) => Ok((fs::read_to_string(p)?, p.display().to_string())),
        None => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s)?;
            Ok((s, "<stdin>".into()))
        }
    }
}
