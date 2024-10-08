use std::str::FromStr;

use kernel::{
    id::UserId,
    model::auth::{event::CreateToken, AccessToken},
};
use shared::error::AppError;

use crate::redis::model::{RedisKey, RedisValue};

pub struct UserItem {
    pub user_id: UserId,
    pub password_hash: String,
}

pub struct AuthorizationKey(String);
pub struct AuthorizationUserId(UserId);

pub fn from(event: CreateToken) -> (AuthorizationKey, AuthorizationUserId) {
    (
        AuthorizationKey(event.access_token),
        AuthorizationUserId(event.user_id),
    )
}

impl From<AuthorizationKey> for AccessToken {
    fn from(key: AuthorizationKey) -> Self {
        Self(key.0)
    }
}

impl From<AccessToken> for AuthorizationKey {
    fn from(token: AccessToken) -> Self {
        Self(token.0)
    }
}

impl From<&AccessToken> for AuthorizationKey {
    fn from(token: &AccessToken) -> Self {
        Self(token.0.to_string())
    }
}

impl RedisKey for AuthorizationKey {
    type Value = AuthorizationUserId;

    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl RedisValue for AuthorizationUserId {
    fn inner(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for AuthorizationUserId {
    type Error = AppError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(UserId::from_str(&s).map_err(|e| {
            AppError::ConversionEntityError(e.to_string())
        })?))
    }
}

impl AuthorizationUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}
