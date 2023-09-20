// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
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

type Object = serde_json::Map<String, serde_json::Value>;

#[derive(Serialize)]
pub struct MessageData {
    pub id: String,
    pub stream_name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub position: i64,
    pub global_position: i64,
    pub metadata: Option<Object>,
    pub data: Object,
    pub time: String,
}

#[command]
async fn messages(stream_name: String, pool: State<'_, PgPool>) -> Result<Vec<MessageData>, ()> {
    #[derive(FromRow)]
    pub struct MessageRow {
        pub id: String,
        pub stream_name: String,
        #[sqlx(rename = "type")]
        pub type_name: String,
        pub position: i64,
        pub global_position: i64,
        pub metadata: Option<String>,
        pub data: String,
        pub time: String,
    }

    const POSITION: i64 = 0;
    const BATCH_SIZE: i64 = 100;
    const CORRELATION: Option<String> = None;
    const CONSUMER_GROUP_MEMBER: Option<i64> = None;
    const CONSUMER_GROUP_SIZE: Option<i64> = None;
    let mut condition = None;

    let rows = if stream_name.contains('*') {
        let category = stream_name.split('-').next();
        let pattern = stream_name.replace('*', "%");
        let escaped_pattern = escape_literal(&pattern);
        condition = Some(format!("stream_name like {escaped_pattern}"));

        sqlx::query_as::<_, MessageRow>("SELECT * from message_store.get_category_messages($1::varchar, $2::bigint, $3::bigint, $4::varchar, $5::bigint, $6::bigint, $7::varchar);")
            .bind(category)
            .bind(POSITION)
            .bind(BATCH_SIZE)
            .bind(CORRELATION)
            .bind(CONSUMER_GROUP_MEMBER)
            .bind(CONSUMER_GROUP_SIZE)
            .bind(condition)
            .fetch_all(pool.inner())
            .await
            .expect("Query message data for category")
    } else {
        sqlx::query_as::<_, MessageRow>("SELECT * from message_store.get_stream_messages($1::varchar, $2::bigint, $3::bigint, $4::varchar)")
            .bind(stream_name)
            .bind(POSITION)
            .bind(BATCH_SIZE)
            .bind(condition)
            .fetch_all(pool.inner())
            .await
            .expect("Query message data for stream")
    };

    let messages = rows
        .into_iter()
        .map(|message_row| {
            let data = serde_json::from_str(&message_row.data).unwrap();
            let metadata = message_row
                .metadata
                .map(|metadata| serde_json::from_str(&metadata).unwrap());

            MessageData {
                id: message_row.id,
                stream_name: message_row.stream_name,
                type_name: message_row.type_name,
                position: message_row.position,
                global_position: message_row.global_position,
                metadata,
                data,
                time: message_row.time,
            }
        })
        .collect();

    Ok(messages)
}

fn escape_literal(string: &str) -> String {
    let mut escaped = "'".to_string();
    let mut has_backslash = false;

    for c in string.chars() {
        match c {
            '\'' => {
                escaped.push(c.clone());
                escaped.push(c);
            }
            '\\' => {
                escaped.push(c.clone());
                escaped.push(c);
                has_backslash = true;
            }
            _ => {
                escaped.push(c);
            }
        };
    }

    escaped.push('\'');

    if has_backslash {
        let e = " E".to_string();
        escaped = e + &escaped;
    }

    escaped
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
        .invoke_handler(tauri::generate_handler![active_stream_names, messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
