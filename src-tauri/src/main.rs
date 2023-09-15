// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::FromRow;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tauri::{command, State};

#[command]
async fn active_stream_names(pool: State<'_, PgPool>) -> Result<Vec<String>, ()> {
    #[derive(FromRow)]
    pub struct Message {
        stream_name: String,
    }

    let messages = sqlx::query_as::<_, Message>(
        "SELECT DISTINCT stream_name, MAX(global_position)
         FROM messages
         GROUP BY stream_name
         ORDER BY max(global_position) DESC LIMIT 25",
    )
    .fetch_all(pool.inner())
    .await
    .expect("Query messages for stream names");

    let stream_names = messages
        .into_iter()
        .map(|message| message.stream_name)
        .collect();

    Ok(stream_names)
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://message_store:@localhost/message_store")
        .await
        .expect("Pool to be connected");

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![active_stream_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
