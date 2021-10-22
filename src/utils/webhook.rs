use minreq as request;

pub struct Webhook {
    url: String,
}

impl Webhook {
    pub fn new(url: &str) -> Self {
        Webhook { url: url.to_string() }
    }

    pub fn send(&self, message: String) -> bool {
        let payload = format!(r#"{{"content": "{}"}}"#, message);
        request::post(&self.url)
            .with_header("Content-Type", "application/json")
            .with_body(payload)
            .send()
            .is_ok()
    }
}
