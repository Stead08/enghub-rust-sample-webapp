use anyhow::{self, Context};

use domain::user::User;
use error::AppError;
use usecase::create_user::CreateUserCommand;

use crate::user::v1::{CreateUserRequest, CreateUserResponse, User as PgUser};

impl TryFrom<CreateUserRequest> for CreateUserCommand {
    type Error = anyhow::Error;

    fn try_from(request: CreateUserRequest) -> anyhow::Result<Self> {
        let CreateUserRequest { name } = request;
        let cmd = CreateUserCommand::builder()
            .name(
                name.try_into()
                    .with_context(|| AppError::InvalidArgument(format!("invalid name")))?,
            )
            .build();
        Ok(cmd)
    }
}

impl From<User> for CreateUserResponse {
    fn from(user: User) -> Self {
        Self {
            user: Some(user.into()),
        }
    }
}

impl From<User> for PgUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}
