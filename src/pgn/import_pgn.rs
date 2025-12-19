use crate::ChessApp;
use crate::Board;
use crate::Coord;
use crate::gui::chessapp_struct::History;

impl ChessApp {

    pub fn import_pgn(&mut self) -> Result<(), String> {
        let mut history = History::new(); 
        let lines: Vec<String> = self
            .pgn_input
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        if let Some(nl) = lines.iter().position(|l| l == "\n") {
            for i in 0..nl {
                history.headers.push(lines[i].clone());
            }
            history.parse_moves(lines, nl)?;
        } else {
            return Err(format!("No new line found to separate headers and san code"));
        }
        Ok(())
    }
}

impl History {


    fn parse_moves(&mut self, lines: Vec<String>, nl: usize) -> Result<(), String> {
        self.history_san = lines.into_iter().collect();

        let moves: Vec<String> = self
            .history_san
            .split(". ")
            .map(|s| s.to_string())
            .collect();

        for m in moves {
            let mv = {
                let gamestate = self.snapshots.last()
                    .ok_or("No gamestate available")?;
                self.get_move_from_san(&m, gamestate)
            };

            if let Some((from, to)) = mv {
                if let Some(gamestate) = self.snapshots.last_mut() {
                    if let Err(e) = gamestate.try_move(&from, &to) {
                        log::debug!("Illegal move: {}", e);
                    } else {
                            //creer le snapshot si inexistant
                            //check draw
                            //update board
                            //impossible mate check
                            //update castles
                            //add hash
                            //last move
                            //increment turn
                            //events_checks
                            //prev board pour promote
                    }
                }
            }
        }

        Ok(())
    }

    fn get_move_from_san(&self, chess_move: &str, gamestate: &Board) -> Option<(Coord, Coord)> {
        None 
    }
}
