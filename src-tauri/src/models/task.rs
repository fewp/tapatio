use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "REWARD_TYPE")]
#[serde(rename_all = "lowercase")]
pub enum RewardType {
    #[sqlx(rename = "pyroxene")]
    Pyroxene,
    #[sqlx(rename = "eleph")]
    Eleph,
    #[sqlx(rename = "credits")]
    Credits,
    #[sqlx(rename = "ap")]
    AP,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub category: String, // "daily" | "weekly" | "challenge" | "achievement";
    pub title: String,
    pub progress_current: i32,
    pub progress_total: i32,
    pub reward_type: RewardType, // "pyroxene" | "eleph" | "credits"
    pub reward_amount: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub category: String,
    pub progress_current: i32,
    pub progress_total: i32,
    pub reward_type: RewardType,
    pub reward_amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]   
    pub reward_type: Option<RewardType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_amount: Option<i32>,
}

impl CreateTask {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".into());
        }
        if self.title.len() > 200 {
            return Err("Title cannot exceed 200 characters".into());
        }
        if self.progress_total <= 0 {
            return Err("Progress total must be greater than zero".into());
        }
        if self.reward_amount < 0 {
            return Err("Reward amount cannot be negative".into());
        }
        Ok(())
    }
}

impl UpdateTask {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                return Err("Title cannot be empty".into());
            }
            if title.len() > 200 {
                return Err("Title cannot exceed 200 characters".into());
            }
        }
        Ok(())
    }

    pub fn is_all_none(&self) -> bool {
        self.title.is_none()
            && self.category.is_none()
            && self.progress_current.is_none()
            && self.progress_total.is_none()
            && self.reward_type.is_none()
            && self.reward_amount.is_none()
    }

    pub fn has_changes(&self) -> bool {
        !self.is_all_none()
    }
}
