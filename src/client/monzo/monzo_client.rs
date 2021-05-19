use crate::client::monzo::mozno_context::MonzoContext;

struct MonzoClient {
    context: MonzoContext
}

impl MonzoClient {
    fn new() -> MonzoClient {
        MonzoClient { context: MonzoContext::new() }
    }

    
}