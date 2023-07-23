use reminder_bot::notify::Notify;
use reminder_bot::reminder::Reminder;
use reqwest;
use serde_json;
use std::error::Error;

pub struct Telegram {
    client: reqwest::blocking::Client,
    url: String,
    bot_id: String,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub update_id: u64,
    pub from: String,
    pub chat_id: i64,
    pub date: u64,
    pub text: String,
}

fn parse_update(update: &serde_json::Value) -> Event {
    Event {
        update_id: update["update_id"].as_u64().unwrap(),
        from: update["message"]["from"]["username"]
            .as_str()
            .unwrap()
            .to_string(),
        chat_id: update["message"]["chat"]["id"].as_i64().unwrap(),
        date: update["message"]["date"].as_u64().unwrap(),
        text: update["message"]["text"].as_str().unwrap().to_string(),
    }
}

fn parse_updates(updates: serde_json::Value) -> Vec<Event> {
    let serde_json::Value::Array(ref entries) = updates["result"] else {
        panic!("Not a vector");
    };

    entries.iter().map(parse_update).collect()
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

    pub fn get_updates(&self, last_id: Option<u64>) -> Result<Vec<Event>, reqwest::Error> {
        let base_url = &self.url;
        let bot_id = &self.bot_id;
        let url = format!("{base_url}/bot{bot_id}/getUpdates");

        let params = [("offset", last_id.unwrap_or(0)), ("timeout", 60)];
        let res = self.client.post(&url).form(&params).send()?;
        let json = res.json::<serde_json::Value>()?;
        Ok(parse_updates(json))
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
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn send_test() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/botsecret-bot-id/sendMessage")
            .create();
        let telegram = Telegram::new("secret-bot-id".to_string(), Some(server.url()));
        let res = telegram.send("chat-id", "test message");

        mock.assert();
        assert!(res.is_ok());
    }

    #[test]
    fn get_updates_test() {
        let body = read_to_string("samples/getUpdates.json").unwrap();
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/botsecret-bot-id/getUpdates")
            .with_body(body)
            .create();
        let telegram = Telegram::new("secret-bot-id".to_string(), Some(server.url()));
        let res = telegram.get_updates(Some(0));

        mock.assert();
        assert!(res.is_ok());

        let events = res.unwrap();
        assert_eq!(events.len(), 2);

        assert_eq!(
            events[0],
            Event {
                update_id: 10,
                from: String::from("user-tag"),
                chat_id: -7,
                date: 1690096028,
                text: String::from("/later 1h message 1"),
            }
        );

        assert_eq!(
            events[1],
            Event {
                update_id: 11,
                from: String::from("user-tag"),
                chat_id: -78,
                date: 1690096064,
                text: String::from("/later 2s message 2"),
            }
        )
    }
}
