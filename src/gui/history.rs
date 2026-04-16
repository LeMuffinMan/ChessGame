use crate::gui::game_state_struct::GameState;

pub struct History {
    pub snapshots: Vec<GameState>,
    //pub coord: Vec<Option<coord>, Option<coord>>,
    pub history_san: String,
}

impl History {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            history_san: String::new(),
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
