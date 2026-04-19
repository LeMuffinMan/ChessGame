#[derive(Clone, PartialEq)]
pub enum End {
    Checkmate,
    TimeOut,
    Pat,
    Draw,
    Resign,
}
