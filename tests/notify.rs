use reminder_bot::id::ID;
use reminder_bot::notify::Notify;
use reminder_bot::reminder::Reminder;
use std::error::Error;
use uuid::Uuid;

struct Notifier;

impl Notify for Notifier {
    fn notify(&self, _reminder: &Reminder) -> Result<&Self, Box<dyn Error>> {
        Ok(self)
    }
}

#[test]
fn test_compilation() {
    let uuid = Uuid::new_v4();

    let id = ID {
        description: "test_id".to_string(),
        uuid: uuid,
    };

    assert!(Notifier
        .notify(&Reminder {
            id: id,
            chat_id: "1234".to_string(),
            message: "hello".to_string(),
        })
        .is_ok());
}
