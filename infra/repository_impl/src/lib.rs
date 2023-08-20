use anyhow::{self, Context};
use async_trait::async_trait;
use db_schema::users;
use derive_new::new;
use diesel::{
    pg::{upsert::excluded, PgConnection},
    prelude::*,
    r2d2::ConnectionManager,
    Insertable, Queryable,
};
use domain::user::{User, UserId};
use domain::user_repository::UserRepository;
use error::AppError;
use r2d2::Pool;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
struct UserRecord {
    pub id: String,
    pub name: String,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> UserRecord {
        UserRecord {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}

impl TryFrom<UserRecord> for User {
    type Error = anyhow::Error;

    fn try_from(user: UserRecord) -> anyhow::Result<User> {
        let UserRecord { id, name } = user;

        User::reconstruct(id, name)
    }
}

#[derive(new)]
pub struct UserRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let user = UserRecord::from(user);
            let mut conn = self.pool.get()?;

            diesel::insert_into(users::table)
                .values(user)
                .on_conflict(users::id)
                .do_update()
                .set(users::name.eq(excluded(users::name)))
                .execute(&mut conn)
                .with_context(|| {
                    AppError::Internal("failed to insert or update user".to_string())
                })?;

            Ok(())
        })
    }

    async fn get_by_ids(&self, ids: &[UserId]) -> anyhow::Result<Vec<User>> {
        tokio::task::block_in_place(|| {
            let mut conn = self.pool.get()?;

            let users: Vec<UserRecord> = users::table
                .filter(users::id.eq_any(ids.iter().map(|id| id.to_string())))
                .select((users::id, users::name))
                .load(&mut conn)
                .with_context(|| AppError::Internal("failed to load users".to_string()))?;

            let users = users
                .into_iter()
                .map(User::try_from)
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| AppError::Internal("failed to convert users".to_string()))?;

            Ok(users)
        })
    }
}
