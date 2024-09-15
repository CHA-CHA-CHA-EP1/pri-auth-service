use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Serialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "userstatus")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum UserStatus {
    ACTIVE,
    INACTIVE,
    Err
}

impl TryFrom<&str> for UserStatus {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ACTIVE" => Ok(UserStatus::ACTIVE),
            "INACTIVE" => Ok(UserStatus::INACTIVE),
            _ => Err("Invalid user status")
        }
    }
}

impl<'de> Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ACTIVE" => Ok(UserStatus::ACTIVE),
            "INACTIVE" => Ok(UserStatus::INACTIVE),
            _ => Ok(UserStatus::Err)
        }
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::INACTIVE
    }
}
