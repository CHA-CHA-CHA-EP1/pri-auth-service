use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Serialize, Clone, Type)]
#[sqlx(type_name = "UserStatus")]
pub enum UserStatus {
    ACTIVE,
    INACTIVE,
    Err
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
