use crate::gui::end::End;

#[derive(PartialEq)]
pub enum AppMode {
    Versus(Option<End>),
    Replay,
    Lobby,
}
