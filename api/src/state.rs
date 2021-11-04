use models::repository::UserRepository;

pub(crate) struct State {
    db: Box<dyn UserRepository + Send + Sync + 'static>,
}

impl State {
    pub(crate) async fn new(
        repository: Box<dyn UserRepository + Send + Sync + 'static>,
    ) -> tide::Result<Self> {
        Ok(Self { db: repository })
    }

    pub fn db(&self) -> &Box<dyn UserRepository + Send + Sync + 'static> {
        &self.db
    }
}
