use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::Otp;
use crate::modules::identity::ports::outbound::OtpServicePort;
use secrecy::{ExposeSecret, SecretString};
use totp_rs::{Algorithm, TOTP};

pub struct TotpOtpService {
    app_secret: SecretString,
}

impl TotpOtpService {
    pub fn new(app_secret: SecretString) -> Self {
        Self { app_secret }
    }

    fn secret_bytes(&self, identity: &Identity) -> Vec<u8> {
        format!(
            "{}:{}:{}",
            self.app_secret.expose_secret(),
            identity.id(),
            identity.email()
        )
        .into_bytes()
    }
}

impl OtpServicePort for TotpOtpService {
    fn generate_otp(&self, identity: &Identity) -> Result<Otp, IdentityError> {
        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 300, self.secret_bytes(identity))
            .map_err(|err| IdentityError::InternalError(format!("Failed to configure OTP generator: {err}")))?;

        let token = totp
            .generate_current()
            .map_err(|err| IdentityError::InternalError(format!("Failed to generate OTP: {err}")))?;

        let otp = Otp::new(token)?;

        Ok(otp)
    }

    fn validate_otp(&self, otp: &Otp) -> Result<bool, IdentityError> {
        todo!()
    }
}
