use anyhow::{bail, Context};
use derive_getters::Getters;
use std::fmt::Formatter;
use std::str::FromStr;
use uuid::Uuid;

use error::AppError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self {
            id: UserId::new(),
            name,
        }
    }
}

impl User {
    /// データベースに保存されたユーザをドメインのエンティティとして再構築する
    pub fn reconstruct(id: String, name: String) -> anyhow::Result<User> {
        let id = id.parse().with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid id".to_string())
        })?;

        let name = name.try_into().with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid name".to_string())
        })?;

        Ok(User { id, name })
    }
}
#[derive(Clone, Debug, Default)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .parse()
            .with_context(|| AppError::InvalidArgument("invalid user id".to_string()))?;
        Ok(UserId(id))
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> anyhow::Result<Self> {
        // ユーザ名は半角英字のみ使用可能
        if name.chars().any(|char| !char.is_ascii_alphabetic()) {
            bail!(AppError::InvalidArgument(
                "ユーザ名は半角英字のみ使用可能です".to_string()
            ));
        }

        // ユーザ名は2文字以上10文字以下である必要がある
        if !(2..10).contains(&name.len()) {
            bail!(AppError::InvalidArgument(
                "ユーザ名は2文字以上10文字以下である必要があります".to_string()
            ));
        }

        Ok(Self(name))
    }
}

impl TryFrom<String> for UserName {
    type Error = anyhow::Error;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        UserName::new(name)
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
