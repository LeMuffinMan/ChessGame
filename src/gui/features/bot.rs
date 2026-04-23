#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PlayerType {
    Human,
    Bot(BotDifficulty),
}
