// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod repository;

use std::env;
use tokio::runtime::Runtime;

use crate::database::{
    seeding::seed_tasks,
    connection::init_pool_from_env,
};
fn main() {
  for (i, arg) in env::args().enumerate() {
    if i == 1 && arg == "seed" {
      println!("🌱 SEEDING DATABASE!");

      let rt = Runtime::new()
          .expect("Failed to create Tokio runtime");

      rt.block_on(async {
          let pool = init_pool_from_env()
              .await
              .expect("Failed to init database");

          seed_tasks(&pool)
              .await
              .expect("Database seeding failed");
      });

      println!("🌸 SEEDED DATABASE!");
      return;
    }
  }

  app_lib::run();

}
