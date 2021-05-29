extern crate log;
use crate::error::GenError;
use std::process::exit;
use log::{error};
use std::env;

pub fn get_required_env(name: &str) -> String {
    match env::var(name) {
        Ok(v) => v,
        Err(err) => {
            error!("Environment variable `{}` not found", name);
            exit_with_error(GenError::from(err))
        }
    }
}

pub fn exit_with_error(e: GenError) -> ! {
    error!("Exiting with fatal error {}", e);
    exit(1);
}