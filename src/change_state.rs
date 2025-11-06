use std::collections::HashMap;

use crate::state::State;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::templates::admin_dashboard::AdminDashboardTemplate;

#[derive(Deserialize)]
pub struct ChangeStateForm {
    pub key: String,
}

pub async fn change_state(
    form: web::Form<ChangeStateForm>,
    pool: actix_web::web::Data<SqlitePool>,
) -> Result<impl Responder, actix_web::Error> {
    let key = &form.key;

    let row = sqlx::query("SELECT state FROM applicants WHERE key = ?")
        .bind(key)
        .fetch_optional(pool.get_ref())
        .await;

    match row {
        Ok(Some(row)) => {
            let state: i32 = row.get("state");

            if state == 1 {
                let _result = sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(2)
                    .bind(key)
                    .execute(pool.get_ref())
                    .await;
            } else if state == 2 {
                let _result = sqlx::query("UPDATE applicants SET state = ? WHERE key = ?")
                    .bind(1)
                    .bind(key)
                    .execute(pool.get_ref())
                    .await;
            }
        }
        Ok(None) => {
            return Ok(AdminDashboardTemplate {
                applicants: Vec::new(),
            });
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Ok(AdminDashboardTemplate {
                applicants: Vec::new(),
            });
        }
    }

    let mut users = HashMap::new();

    let rows = sqlx::query("SELECT key, value, state, created_at FROM applicants")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    for row in rows {
        let key: String = row.get("key");
        let value: String = row.get("value");
        let state: i32 = row.get("state");
        let created_at: String = row.get("created_at");

        let user_created_at: chrono::NaiveDateTime =
            chrono::NaiveDateTime::parse_from_str(created_at.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let real_state = match state {
            1 => State::Fresh,
            2 => State::Old,
            _ => State::None,
        };

        let user = format!(
            "{},  {}  state: {},  created at: {}",
            key, value, real_state, user_created_at
        );
        users.insert(key, user);
    }

    Ok(AdminDashboardTemplate {
        applicants: users.into_iter().collect::<Vec<_>>(),
    })
}
