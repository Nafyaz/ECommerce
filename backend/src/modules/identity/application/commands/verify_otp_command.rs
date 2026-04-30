use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, Otp};
use secrecy::SecretString;

pub struct VerifyOtpCommand {
    email: Email,
    otp: Otp,
}

impl VerifyOtpCommand {
    pub fn new(email: String, otp: String) -> Result<Self, IdentityError> {
        let email = Email::new(email)?;
        let otp = Otp::new(otp)?;

        Ok(Self { email, otp })
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn otp(&self) -> &Otp {
        &self.otp
    }
}
