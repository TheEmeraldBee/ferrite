use std::sync::{Arc, Mutex};

use widgetui::State;

#[derive(State)]
pub struct MakefileState {
    pub targets: Vec<String>,
    pub run_target: Arc<Mutex<Option<String>>>,
    pub selected: usize,
    pub last_target: Option<String>,
}
