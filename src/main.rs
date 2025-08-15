mod board;
use board::Board;

fn main() {
    let board = Board::init_board();

    board.print();
}
