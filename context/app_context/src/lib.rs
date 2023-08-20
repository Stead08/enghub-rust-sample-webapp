use domain::user_repository::ProvideUserRepository;
use repository_impl::UserRepositoryImpl;

pub struct AppContext {
    pub user_repository: UserRepositoryImpl,
}

impl ProvideUserRepository for AppContext {
    type Repository = UserRepositoryImpl;

    fn provide(&self) -> &Self::Repository {
        &self.user_repository
    }
}
