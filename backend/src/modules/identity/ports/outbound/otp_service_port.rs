use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::Otp;

pub trait OtpServicePort: Send + Sync {
    fn generate_otp(&self, identity: &Identity) -> Result<Otp, IdentityError>;

    fn validate_otp(&self, otp: &Otp) -> Result<bool, IdentityError>;
}
