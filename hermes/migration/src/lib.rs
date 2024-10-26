pub use sea_orm_migration::prelude::*;

mod m20240824_010034_create_user_table;
mod m20240824_010041_create_conversation_table;
mod m20240824_010043_create_message_table;
mod m20240824_045520_create_userconvo_table;
mod m20240824_184203_add_default_values_to_all;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240824_010034_create_user_table::Migration),
            Box::new(m20240824_010041_create_conversation_table::Migration),
            Box::new(m20240824_010043_create_message_table::Migration),
            Box::new(m20240824_045520_create_userconvo_table::Migration),
            Box::new(m20240824_184203_add_default_values_to_all::Migration),
        ]
    }
}
