use std::fs::File;

use crate::{BasilModule, ModuleError, ModuleResponse};
pub struct LogsModule {}

impl LogsModule {
    pub fn new() -> LogsModule {
        {
            let _file = File::create("logs.db");
        }

        let connection = rusqlite::Connection::open("logs.db").unwrap();

        let query = "SELECT name FROM sqlite_master WHERE type='table';";

        let mut log_table_names = connection.prepare(query).unwrap();

        println!(
            "Current tables: {}",
            log_table_names.expanded_sql().unwrap()
        );

        LogsModule {}
    }
}

impl BasilModule for LogsModule {
    fn get_description(&self) -> String {
        "Module for logging activities and revisiting past activites".to_string()
    }
    fn get_keywords(&self) -> Vec<String> {
        vec!["log".to_string(), "logs".to_string()]
    }
    fn get_examples(&self) -> Vec<String> {
        vec![
            "what do my logs say about yesterday's work out?".to_string(),
            "log 200 calories eaten".to_string(),
            "log a 2 mile run".to_string(),
            "what do my logs say about what I ate today?".to_string(),
            "create a log for weightlifting".to_string(),
        ]
    }

    fn run(&self, request: String) -> Result<ModuleResponse, ModuleError> {
        println!("Received request for logs: {}", request);

        Ok(ModuleResponse {
            is_finished: true,
            text_response: "Log saved.".to_string(),
        })
    }

    fn get_module_name(&self) -> String {
        "logs".to_string()
    }
}
