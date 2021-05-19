use std::env;
use std::env::VarError;
use crate::util::{exit_with_error, get_required_env};
use crate::error::GenError;

const REDIRECT_URI_ENV_NAME: &str = "MONZO_REDIRECT_URI";
const CLIENT_SECRET_ENV_NAME: &str = "MONZO_CLIENT_SECRET";
const CLIENT_ID_ENV_NAME: &str = "MONZO_CLIENT_ID";
const ACCOUNT_ID_ENV_NAME: &str = "MONZO_ACCOUNT_ID";
const BASE_URL: &str = "https://api.monzo.com";
const BASE_AUTH_URL: &str = "https://auth.monzo.com";

pub struct MonzoContext {
    pub redirect_uri: String,
    pub client_secret: String,
    pub client_id: String,
    pub account_id: String,
    pub base_url: String,
    pub base_auth_url: String
}

impl MonzoContext {
    pub fn new() -> MonzoContext {
        MonzoContext {
            redirect_uri: get_required_env(REDIRECT_URI_ENV_NAME),
            client_secret: get_required_env(CLIENT_SECRET_ENV_NAME),
            client_id: get_required_env(CLIENT_ID_ENV_NAME),
            account_id: get_required_env(ACCOUNT_ID_ENV_NAME),
            base_url: BASE_URL.to_string(),
            base_auth_url: BASE_AUTH_URL.to_string()
        }
    }
}