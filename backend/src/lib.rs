use logs::LogsModule;
use notes::NotesModule;

pub mod logs;
pub mod notes;

pub enum Modules {
    Notes(NotesModule),
    Logs(LogsModule),
}

pub trait BasilModule {
    fn get_module_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_keywords(&self) -> Vec<String>;
    fn get_examples(&self) -> Vec<String>;
    fn run(&self, request: String) -> Result<ModuleResponse, ModuleError>;
}

pub struct ModuleResponse {
    pub is_finished: bool,
    pub text_response: String,
}

pub struct ModuleError {
    pub num_reties: u8,
    pub error_text: String,
}
