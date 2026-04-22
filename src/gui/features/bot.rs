#[derive(PartialEq, Debug)]
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}
#[derive(PartialEq, Debug)]
pub enum PlayerType {
    Human,
    Bot(BotDifficulty),
}
