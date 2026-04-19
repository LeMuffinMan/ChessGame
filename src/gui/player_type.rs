use crate::gui::bot_difficulty::BotDifficulty;

#[derive(PartialEq, Debug)]
pub enum PlayerType {
    Human,
    Bot(BotDifficulty),
}
