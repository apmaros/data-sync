use crate::client::monzo::mozno_context::MonzoContext;
use uuid::Uuid;

pub struct MonzoClient {
    context: MonzoContext
}

impl MonzoClient {
    pub fn new() -> MonzoClient {
        MonzoClient { context: MonzoContext::new() }
    }

    pub fn get_auth_url(&self) -> String {
        format!(
            "{}/?client_id={}&redirect_uri={}&response_type=code&state={}",
            self.context.base_auth_url,
            self.context.client_id,
            self.context.redirect_uri,
            Uuid::new_v4()
        )
    }

}