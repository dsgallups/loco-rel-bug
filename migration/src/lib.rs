#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250120_152959_contributors;
mod m20250120_153120_posts;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250120_152959_contributors::Migration),
            Box::new(m20250120_153120_posts::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}