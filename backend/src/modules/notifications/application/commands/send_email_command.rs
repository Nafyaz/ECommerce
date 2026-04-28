use crate::modules::notifications::NotificationError;
use crate::modules::notifications::domain::value_objects::{NotificationBody, Recipient, Subject};

pub struct SendEmailCommand {
    recipient: Recipient,
    subject: Subject,
    body: NotificationBody,
}

impl SendEmailCommand {
    pub fn new(recipient: String, subject: String, body: String) -> Result<Self, NotificationError> {
        let recipient = Recipient::email(recipient)?;
        let subject = Subject::new(subject)?;
        let body = NotificationBody::new(body)?;

        Ok(Self {
            recipient,
            subject,
            body,
        })
    }

    pub fn recipient(&self) -> &Recipient {
        &self.recipient
    }

    pub fn subject(&self) -> Subject {
        self.subject.clone()
    }

    pub fn body(&self) -> NotificationBody {
        self.body.clone()
    }
}
