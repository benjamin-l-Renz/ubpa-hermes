use chrono::Local;
use sqlx::{Row, SqlitePool};
use tokio::time::{Duration, interval};

/// Runs every 24 hours on a background thread
pub async fn background_worker(pool: SqlitePool) {
    let mut ticker = interval(Duration::from_hours(24));

    loop {
        ticker.tick().await;
        println!("Running scheduled task...");

        // Get current time as UTC
        let current_time = Local::now().naive_local();

        // Get the rows
        let rows = sqlx::query("SELECT key, value, state, created_at FROM applicants")
            .fetch_all(&pool)
            .await
            .unwrap();

        for row in rows {
            let key: String = row.get("key");
            let created_at: String = row.get("created_at");
            let state: i32 = row.get("state");
            let user_created_at: chrono::NaiveDateTime =
                chrono::NaiveDateTime::parse_from_str(created_at.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap();

            let duration_in_db = current_time - user_created_at;

            // If the duration equals one day and the state is 'fresh' it will be set to 'old'
            if duration_in_db.num_days() == 1 && state == 1 {
                let new_state = state + 1;

                // Update the state in the db
                sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(new_state)
                    .bind(key)
                    .execute(&pool)
                    .await
                    .unwrap();
            }
        }
    }
}
