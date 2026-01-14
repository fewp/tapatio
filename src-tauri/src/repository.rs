use sqlx::PgPool;

use crate::models::{CreateTask, Task, UpdateTask};

/// List all tasks ordered by creation date (newest first)
pub async fn list(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT
            id,
            category, 
            title,
            progress_current,
            progress_total,
            reward_type,
            reward_amount,
            created_at,
            updated_at,
            completed_at
        FROM tasks
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

/// Create a new task (timestamps in UTC, completed_at = NULL)
pub async fn create(pool: &PgPool, payload: CreateTask) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (category, title, progress_current, progress_total, reward_type, reward_amount, created_at, updated_at)
        VALUES (
            $1, $2, $3, $4, $5, $6, NOW() AT TIME ZONE 'utc', NOW() AT TIME ZONE 'utc' 
        )
        RETURNING
            id,
            category,
            title,
            progress_current,
            progress_total,
            reward_type,
            reward_amount,
            created_at,
            updated_at,
            deleted_at,
            completed_at
        "#
    )
    .bind(payload.category)          // TEXT
    .bind(payload.title)             // TEXT
    .bind(payload.progress_current)  // INT
    .bind(payload.progress_total)   // INT
    .bind(payload.reward_type)       // TEXT
    .bind(payload.reward_amount)     // INT
    .fetch_one(pool)
    .await
}

/// Update an existing task (partial patch)
pub async fn update(pool: &PgPool, id: i64, patch: UpdateTask) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET
          category     = COALESCE($2, category),
          title        = COALESCE($3, title),
          progress_current = COALESCE($4, progress_current),
          progress_total   = COALESCE($5, progress_total),
          reward_type  = COALESCE($6, reward_type),
          reward_amount = COALESCE($7, reward_amount),
          updated_at   = NOW() AT TIME ZONE 'utc'
        WHERE id = $1
        RETURNING
          id,
          category,
          title,
          progress_current,
          progress_total,
          reward_type,
          reward_amount,
          created_at,
          updated_at,
          completed_at
        "#
    )
    .bind(id)                          // $1
    .bind(patch.category)             // $2 Option<TEXT>
    .bind(patch.title)                // $3 Option<TEXT>
    .bind(patch.progress_current)     // $4 Option<INT>
    .bind(patch.progress_total)       // $5 Option<INT>
    .bind(patch.reward_type)          // $6 Option<TEXT>
    .bind(patch.reward_amount)        // $7 Option<INT>
    .fetch_one(pool)
    .await
}

/// Delete a task by ID
/// Returns 1 if deleted, 0 if not found
pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"DELETE FROM tasks WHERE id = $1"#
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}