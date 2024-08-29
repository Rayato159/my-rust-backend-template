use chrono::NaiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: None,
            username,
            password,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn to_model(&self) -> crate::models::user::User {
        crate::models::user::User {
            id: self.id.unwrap(),
            username: self.username.clone(),
        }
    }
}
