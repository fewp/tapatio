use sqlx::PgPool;

use crate::repository;
use crate::models::{CreateTask, RewardType};

pub async fn seed_tasks(pool: &PgPool) -> Result<(), sqlx::Error> {
    let tasks = vec![
        CreateTask {
            category: "daily".to_string(),
            title: "Daily login".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::AP,
            reward_amount: 100,
        },
        CreateTask {
            category: "daily".to_string(),
            title: "Hold a lesson 2 times".to_string(),
            progress_current: 0,
            progress_total: 2,
            reward_type: RewardType::AP,
            reward_amount: 50,
        },
        CreateTask {
            category: "daily".to_string(),
            title: "Recharge AP 1 time".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Credits,
            reward_amount: 20_000,
        },
        CreateTask {
            category: "daily".to_string(),
            title: "Complete at least 5 Daily Tasks".to_string(),
            progress_current: 1,
            progress_total: 5,
            reward_type: RewardType::Pyroxene,
            reward_amount: 20,
        },
        CreateTask {
            category: "daily".to_string(),
            title: "Clear a Bounty 3 times".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Eleph,
            reward_amount: 5,
        },
        CreateTask {
            category: "weekly".to_string(),
            title: "Daily Login".to_string(),
            progress_current: 4,
            progress_total: 5,
            reward_type: RewardType::AP,
            reward_amount: 200,
        },
        CreateTask {
            category: "weekly".to_string(),
            title: "Craft 5 times".to_string(),
            progress_current: 3,
            progress_total: 5,
            reward_type: RewardType::Pyroxene,
            reward_amount: 80,
        },
        CreateTask {
            category: "weekly".to_string(),
            title: "Clear a Scrimmage 15 times".to_string(),
            progress_current: 3,
            progress_total: 15,
            reward_type: RewardType::Credits,
            reward_amount: 15_000,
        },
        CreateTask {
            category: "weekly".to_string(),
            title: "Hold a lesson 9 times".to_string(),
            progress_current: 4,
            progress_total: 9,
            reward_type: RewardType::AP,
            reward_amount: 150,
        },
        CreateTask {
            category: "weekly".to_string(),
            title: "Complete a weekly mission at least 6 times".to_string(),
            progress_current: 8,
            progress_total: 15,
            reward_type: RewardType::Eleph,
            reward_amount: 15,
        },
        CreateTask {
            category: "achievement".to_string(),
            title: "Clear a Commission 775 times".to_string(),
            progress_current: 666,
            progress_total: 775,
            reward_type: RewardType::Pyroxene,
            reward_amount: 150,
        },
        CreateTask {
            category: "achievement".to_string(),
            title: "Level up a skill 500 times".to_string(),
            progress_current: 333,
            progress_total: 500,
            reward_type: RewardType::Pyroxene,
            reward_amount: 100,
        },
        CreateTask {
            category: "achievement".to_string(),
            title: "Earn 1,000,000,000 credits".to_string(),
            progress_current: 850_000_000,
            progress_total: 1_000_000_000,
            reward_type: RewardType::Credits,
            reward_amount: 500_000,
        },
        CreateTask {
            category: "achievement".to_string(),
            title: "Enter a Total Assault 150 times".to_string(),
            progress_current: 67,
            progress_total: 150,
            reward_type: RewardType::Pyroxene,
            reward_amount: 150,
        },
        CreateTask {
            category: "achievement".to_string(),
            title: "Level up equipment 1000 times".to_string(),
            progress_current: 420,
            progress_total: 1_000,
            reward_type: RewardType::Eleph,
            reward_amount: 25,
        },
        CreateTask {
            category: "challenge".to_string(),
            title: "Clear Mission 1, Stage 2 Strategy on Hard within 2 turns".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Pyroxene,
            reward_amount: 30,
        },
        CreateTask {
            category: "challenge".to_string(),
            title: "Clear Mission 3, Stage 1 Strategy on Hard within 2 turns".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Pyroxene,
            reward_amount: 40,
        },
        CreateTask {
            category: "challenge".to_string(),
            title: "Clear Mission 7, Stage 6 Strategy on Hard within 2 turns".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Pyroxene,
            reward_amount: 60,
        },
        CreateTask {
            category: "challenge".to_string(),
            title: "Clear Mission 23, Stage 1 Strategy on Hard within 2 turns".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Credits,
            reward_amount: 5_000,
        },
        CreateTask {
            category: "challenge".to_string(),
            title: "Clear Mission 15, Stage 2 Strategy on Hard within 2 turns".to_string(),
            progress_current: 0,
            progress_total: 1,
            reward_type: RewardType::Credits,
            reward_amount: 5_000,
        },
    ];

    for task in tasks {
        repository::create(pool, task).await?;
    }

    Ok(())
}