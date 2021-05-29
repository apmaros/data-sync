use crate::util::get_required_env;

const REDIRECT_URI_ENV_NAME: &str = "MONZO_REDIRECT_URI";
const CLIENT_SECRET_ENV_NAME: &str = "MONZO_CLIENT_SECRET";
const CLIENT_ID_ENV_NAME: &str = "MONZO_CLIENT_ID";
const ACCOUNT_ID_ENV_NAME: &str = "MONZO_ACCOUNT_ID";
const BASE_URL: &str = "https://api.monzo.com";
const AUTH_URL: &str = "https://auth.monzo.com";
const TOKEN_URL: &str = "https://api.monzo.com/oauth2/token";

#[derive(Clone, Debug)]
pub struct MonzoContext {
    pub redirect_uri: String,
    pub client_secret: String,
    pub client_id: String,
    pub account_id: String,
    pub base_url: String,
    pub auth_url: String,
    pub token_url: String
}

impl MonzoContext {
    pub fn new() -> MonzoContext {
        MonzoContext {
            redirect_uri: get_required_env(REDIRECT_URI_ENV_NAME),
            client_secret: get_required_env(CLIENT_SECRET_ENV_NAME),
            client_id: get_required_env(CLIENT_ID_ENV_NAME),
            account_id: get_required_env(ACCOUNT_ID_ENV_NAME),
            base_url: BASE_URL.to_string(),
            auth_url: AUTH_URL.to_string(),
            token_url: TOKEN_URL.to_string()
        }
    }
}
