use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaymentConfig {
    pub stripe_secret_key: String,
    pub webhook_secret: String,
    pub payout_schedule_days: u32,
    pub platform_fee_percentage: f64,
}
