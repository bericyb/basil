use crate::{BasilModule, ModuleError, ModuleResponse};
pub struct NotesModule {}
impl BasilModule for NotesModule {
    fn get_description(&self) -> String {
        "Module for creating, editing, and relaying notes".to_string()
    }
    fn get_keywords(&self) -> Vec<String> {
        vec![
            "notes".to_string(),
            "note".to_string(),
            "memo".to_string(),
            "list".to_string(),
        ]
    }
    fn get_examples(&self) -> Vec<String> {
        vec![
            "what do my notes say about my communications class?".to_string(),
            "make a note about quantum computing".to_string(),
            "make a list of movies I want to watch".to_string(),
        ]
    }

    fn run(&self, request: String) -> Result<ModuleResponse, ModuleError> {
        println!("Received request for notes: {}", request);

        Ok(ModuleResponse {
            is_finished: true,
            text_response: "Note saved.".to_string(),
        })
    }

    fn get_module_name(&self) -> String {
        "notes".to_string()
    }
}
