use chrono::{NaiveDateTime, TimeZone, Utc};

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(username: String, password: String, is_auto_time: bool) -> Self {
        User {
            id: None,
            username,
            password,
            created_at: if is_auto_time {
                Utc::now().naive_utc()
            } else {
                Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                    .unwrap()
                    .naive_utc()
            },
            updated_at: if is_auto_time {
                Utc::now().naive_utc()
            } else {
                Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                    .unwrap()
                    .naive_utc()
            },
        }
    }

    pub fn to_model(&self) -> crate::models::user::User {
        crate::models::user::User {
            id: self.id.unwrap(),
            username: self.username.clone(),
        }
    }
}
