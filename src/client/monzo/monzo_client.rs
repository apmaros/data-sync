use crate::client::monzo::mozno_context::MonzoContext;
use uuid::Uuid;
use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, TokenUrl, AuthUrl, url, RedirectUrl, CsrfToken, AuthorizationCode, TokenResponse, AccessToken};
use log::error;
use crate::util::exit_with_error;
use crate::error::GenError;
use oauth2::url::Url;
use oauth2::reqwest::http_client;
use crate::client::monzo::monzo_error::MonzoError;
use std::sync::Once;

lazy_static! {
    pub static ref MONZO_CLIENT: MonzoClient = MonzoClient::new().expect("Bum");
}

#[derive(Clone, Debug)]
pub struct MonzoClient {
    inner: BasicClient
}

impl MonzoClient {
    pub fn new() -> Result<MonzoClient, GenError> {
        let context = MonzoContext::new();
        let client_id = ClientId::new(context.client_id.clone());
        let client_secret = ClientSecret::new(context.client_secret);
        let token_url= TokenUrl::new(context.token_url)?;
        let auth_url = AuthUrl::new(context.auth_url)?;
        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url)
        ).set_redirect_uri(
            RedirectUrl::new(context.redirect_uri)?
        );

        Ok(MonzoClient { inner: client })
    }

    pub fn get_auth_url(&self) -> (Url, CsrfToken) {
        self.inner.authorize_url(CsrfToken::new_random).url()
    }

    pub fn get_token(&self, code: String) -> Result<AccessToken, MonzoError>{
        let code = AuthorizationCode::new(code);
        match self.inner.exchange_code(code).request(http_client) {
            Ok(resp) => {
                Ok(resp.access_token().clone())
            },
            Err(err) => Err(MonzoError::from(err))
        }
    }
}
