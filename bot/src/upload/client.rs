pub(crate) struct Client {
    req: reqwest::Client,
    base_url: String,
}

impl Client {
    #[inline]
    fn get_user_agent() -> String {
        format!("grrr@{}", crate::ver::VERSION)
    }

    pub fn new(base_url: String) -> Self {
        let req = reqwest::Client::builder()
            .user_agent(Client::get_user_agent())
            .build()
            .expect("failed to create client");

        Client {
            req,
            base_url
        }
    }

    pub async fn ping(&self) -> Result<(), reqwest::Error> {
        self.req.get(&self.base_url).send().await.map(|_| ())
    }
}
