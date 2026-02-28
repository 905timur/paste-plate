/// Tool types available in the application
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Tool {
    Move,
    Transform,
    Selection,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Move
    }
}
