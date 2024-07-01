use std::fmt::Display;
use std::process::exit;

pub fn handle_panic<E>(error: E)
where
    E: Display,
{
    eprintln!("{error}");
    exit(1);
}
