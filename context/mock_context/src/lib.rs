use domain::user_repository::{MockUserRepository, ProvideUserRepository};

#[derive(Debug, Default)]
pub struct MockContext {
    pub user_repository: MockUserRepository,
}

impl ProvideUserRepository for MockContext {
    type Repository = MockUserRepository;

    fn provide(&self) -> &Self::Repository {
        &self.user_repository
    }
}
