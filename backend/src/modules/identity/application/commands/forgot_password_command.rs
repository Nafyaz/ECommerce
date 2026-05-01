use crate::modules::identity::domain::value_objects::Email;

pub struct ForgotPasswordCommand {
    email: Email,
}
