use filler::game::{player, Board, Piece, Player, State};
fn main() {
    let player = player();
    let board = Board::new();
    let players = Player::init(&board);
    let mut state = State::new(board, player, players);
    loop {
        state.make_move(&Piece::new());
        state.update();
    }
}
