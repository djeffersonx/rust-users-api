use sqlx::{types::Uuid};

#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub password: String,
}
