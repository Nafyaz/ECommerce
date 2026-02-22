use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ParseAppEnvError(String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    Dev,
    Test,
    UAT,
    Stage,
    Prod,
}

impl AppEnv {
    pub fn is_production(&self) -> bool {
        *self == Self::Prod
    }

    pub fn is_development(&self) -> bool {
        *self == Self::Dev
    }
}

impl std::fmt::Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dev => write!(f, "Dev"),
            Self::Test => write!(f, "Test"),
            Self::UAT => write!(f, "UAT"),
            Self::Stage => write!(f, "Stage"),
            Self::Prod => write!(f, "Prod"),
        }
    }
}

impl FromStr for AppEnv {
    type Err = ParseAppEnvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "test" => Ok(Self::Test),
            "uat" => Ok(Self::UAT),
            "stage" => Ok(Self::Stage),
            "prod" => Ok(Self::Prod),
            _ => Err(ParseAppEnvError(s.into())),
        }
    }
}
