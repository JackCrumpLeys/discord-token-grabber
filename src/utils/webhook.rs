use ureq as request;
use ureq::json;

pub struct Webhook {
    url: String,
}

impl Webhook {
    pub fn new(url: String) -> Self {
        Webhook { url }
    }

    pub fn send(&self, message: &str) -> bool {
        let payload = json!({ "content": message });
        request::post(self.url.as_str())
            .send_json(payload)
            .is_ok()
    }
}
