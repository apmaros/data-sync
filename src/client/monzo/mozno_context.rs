use std::env;
use std::env::VarError;
use crate::util::{exit_with_error, get_required_env};
use crate::error::GenError;

const REDIRECT_URI_ENV_NAME: &str = "MONZO_REDIRECT_URI";
const CLIENT_SECRET_ENV_NAME: &str = "MONZO_CLIENT_SECRET";
const CLIENT_ID_ENV_NAME: &str = "MONZO_CLIENT_ID";
const ACCOUNT_ID_ENV_NAME: &str = "MONZO_ACCOUNT_ID";
const BASE_URL: &str = "https://api.monzo.com";

pub struct MonzoContext {
    redirect_uri: String,
    client_secret: String,
    client_id: String,
    account_id: String,
    base_url: String
}

impl MonzoContext {
    pub fn new() -> MonzoContext {
        MonzoContext {
            redirect_uri: get_required_env(REDIRECT_URI_ENV_NAME),
            client_secret: get_required_env(CLIENT_SECRET_ENV_NAME),
            client_id: get_required_env(CLIENT_ID_ENV_NAME),
            account_id: get_required_env(ACCOUNT_ID_ENV_NAME),
            base_url: BASE_URL.to_string()
        }
    }
}