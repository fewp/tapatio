use tauri::Manager;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod database;
pub mod models;
pub mod repository;
mod error;

use error::to_user_error;

use crate::models::{CreateTask, Task, UpdateTask};

#[tauri::command]
async fn list_tasks(pool: tauri::State<'_, PgPool>) -> Result<Vec<Task>, String> {
    repository::list(&*pool).await.map_err(to_user_error)
}

// /// Get a single task
// #[tauri::command]
// async fn get_task(pool: tauri::State<'_, PgPool>, id: i64) -> Result<Task, String> {
//     repo::get_by_id(&*pool, id).await.map_err(to_user_error)
// }

// /// Create a todo
// #[tauri::command]
// async fn create_todo(
//     pool: tauri::State<'_, PgPool>,
//     payload: CreateTodo,
// ) -> Result<Todo, String> {
//     // Use your model-level validation
//     payload.validate().map_err(|e| e.to_string())?;
//     repo::create(&*pool, payload).await.map_err(to_user_error)
// }

// /// Patch update a todo
// #[tauri::command]
// async fn update_todo(
//     pool: tauri::State<'_, PgPool>,
//     id: i64,
//     patch: UpdateTodo,
// ) -> Result<Todo, String> {
//     let patch = { patch.validate().map_err(|e| e.to_string())?; patch };
//     if !patch.has_changes() {
//         return Err("No changes provided".to_string());
//     }
//     repo::update(&*pool, id, patch).await.map_err(to_user_error)
// }

// /// Toggle completion explicitly
// /// NOTE: parameter name is `to_completed` to match your current context.
// #[tauri::command]
// async fn toggle_todo(
//     pool: tauri::State<'_, PgPool>,
//     id: i64,
//     to_completed: bool,
// ) -> Result<Todo, String> {
//     repo::toggle(&*pool, id, to_completed).await.map_err(to_user_error)
// }

// /// Delete by id (returns 1 on success, 0 if not found)
// #[tauri::command]
// async fn delete_todo(pool: tauri::State<'_, PgPool>, id: i64) -> Result<u64, String> {
//     let n = repo::delete(&*pool, id).await.map_err(to_user_error)?;
//     if n == 0 {
//         return Err("Todo not found".into());
//     }
//     Ok(n)
// }


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  println!("🔥 Tapatio");

  // Load env from project root first; fallback to current dir
  let _ = dotenvy::from_filename("../.env").or_else(|_| dotenvy::dotenv());
  println!("📄 Loaded environment variables");
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

  tauri::Builder::default()
      .setup(move |app| {
          // Build pool & migrate in Tauri's async runtime
          let pool: PgPool = tauri::async_runtime::block_on(async {
              use std::time::Duration;

              let pool = PgPoolOptions::new()
                  .max_connections(5)
                  .min_connections(1)
                  .acquire_timeout(Duration::from_secs(10))
                  .idle_timeout(Duration::from_secs(600))
                  .max_lifetime(Duration::from_secs(1800))
                  .connect(&database_url)
                  .await
                  .map_err(|e| anyhow::anyhow!("connect failed: {e}"))?;
              println!("✅ Successfully connected to PostgreSQL database");

              println!("Running database migrations...");
              sqlx::migrate!("./src/database/migrations/")
                  .run(&pool)
                  .await
                  .map_err(|e| anyhow::anyhow!("migrations failed: {e}"))?;
              println!("✅ Database migrations completed successfully");

              // Quick sanity ping
              let one: i32 = sqlx::query_scalar("SELECT 1")
                  .fetch_one(&pool)
                  .await
                  .map_err(|e| anyhow::anyhow!("ping failed: {e}"))?;
              println!("✅ Database connection test passed (SELECT 1 = {one})");
              println!("✅ Database initialization complete");

              Ok::<PgPool, anyhow::Error>(pool)
          })?;

          // Put PgPool into Tauri state
          app.manage::<PgPool>(pool);

          // Dev logging plugin
          #[cfg(debug_assertions)]
          app.handle().plugin(
              tauri_plugin_log::Builder::default()
                  .level(log::LevelFilter::Info)
                  .build(),
          )?;

          Ok(())
      })
      .invoke_handler(tauri::generate_handler![
          list_tasks,
        //   create_task,
        //   update_task,
        //   delete_task
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
