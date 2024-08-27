pub mod postgres {
    use std::sync::Arc;

    use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

    use crate::setting::app::Setting;

    pub async fn conn_getting(setting: Arc<Setting>) -> Result<Arc<Pool<Postgres>>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(setting.database.url_getting().as_str())
            .await?;

        Ok(Arc::new(pool))
    }
}
