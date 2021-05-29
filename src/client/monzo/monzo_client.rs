use crate::client::monzo::mozno_context::MonzoContext;
use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, TokenUrl, AuthUrl, RedirectUrl, CsrfToken};
use crate::error::GenError;
use oauth2::url::Url;
use crate::client::monzo::monzo_error::MonzoError;
use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;


#[derive(Clone, Debug)]
pub struct MonzoClient {
    inner: BasicClient,
    context: MonzoContext
}

impl MonzoClient {
    pub fn new() -> Result<MonzoClient, GenError> {
        let context = MonzoContext::new();
        let client = build_client(&context)?;

        Ok(MonzoClient { inner: client, context })
    }

    pub fn get_auth_url(&self) -> (Url, CsrfToken) {
        self.inner.authorize_url(CsrfToken::new_random).url()
    }

    pub fn get_token(&self, code: String) -> Result<String, MonzoError> {
        // todo build params in nicer way - during initialization
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", "oauth2client_0000A4hSiAmlHuPO29gtuL");
        params.insert("client_secret", "mnzconf.DW1PtKJskJk66f0QpJgPhiGPHE7K8A0+VneUOBMokYzUjwGEmsgymQRvd6fvvMSa47R8tfJMIOCmxLJu1QleBA==");
        params.insert("redirect_uri", "http://127.0.0.1:8050/home?from=auth");
        params.insert("code", &code);

        let client = reqwest::blocking::Client::new();
        let token = client.post("https://api.monzo.com/oauth2/token")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()?
            .json::<AccessToken>()?;


        // let access_token = res.get("access_token").unwrap();
        // let client_id = res.get("client_id").unwrap();
        // let expires_in = res.get("expires_in").unwrap();
        // let refresh_token = res.get("refresh_token").unwrap();
        // let user_id = res.get("user_id").unwrap();

        Ok(token.access_token)
    }
}

#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    client_id: String,
    expires_in: u32,
    refresh_token: String,
    user_id: String
}


fn build_client(context: &MonzoContext) -> Result<BasicClient, GenError> {
    let client_id = ClientId::new(context.client_id.clone());
    let client_secret = ClientSecret::new(context.client_secret.clone());
    let token_url= TokenUrl::new(context.token_url.clone())?;
    let auth_url = AuthUrl::new(context.auth_url.clone())?;

    let client: BasicClient = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url)
    ).set_redirect_uri(
        RedirectUrl::new(context.redirect_uri.clone())?
    );

    Ok(client)
}

// todo Idea: implement MonzoAuth trait responsible for building client and implementing auth methods