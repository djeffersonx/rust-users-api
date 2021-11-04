use async_trait::async_trait;
use std::error::Error;

use crate::user::User;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn find_by_id(&self, id: String) -> Result<User, Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn Error>>;
}
