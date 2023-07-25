use reminder_bot::notify::Notify;
use reminder_bot::reminder::Reminder;
use reqwest;
use std::error::Error;

pub struct Telegram {
    client: reqwest::blocking::Client,
    url: String,
    bot_id: String,
}

impl Telegram {
    pub fn new(bot_id: String, url: Option<String>) -> Self {
        Telegram {
            client: reqwest::blocking::Client::new(),
            bot_id: bot_id,
            url: url.unwrap_or("api.telegram.org".to_string()),
        }
    }

    pub fn send(&self, chat_id: &str, text: &str) -> Result<String, reqwest::Error> {
        let base_url = &self.url;
        let bot_id = &self.bot_id;
        let url = format!("{base_url}/bot{bot_id}/sendMessage");

        let params = [
            ("disable_web_page_preview", "true"),
            ("disable_notification", "false"),
            ("parse_mode", "none"),
            ("chat_id", chat_id),
            ("text", text),
        ];

        let res = self.client.post(&url).form(&params).send()?;

        res.text()
    }
}

impl Notify for Telegram {
    fn notify(&self, reminder: &Reminder) -> Result<&Self, Box<dyn Error>> {
        match self.send(&reminder.chat_id, &reminder.message) {
            Err(err) => Err(Box::new(err)),
            Ok(_) => Ok(&self),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito;

    use super::*;

    #[test]
    fn send_test() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/botsecret-bot-id/sendMessage")
            .create();
        let telegram = Telegram::new("secret-bot-id".to_string(), Some(server.url()));
        let res = telegram.send("chat-id", "test message");

        assert!(res.is_ok());
        mock.assert();
    }
}
