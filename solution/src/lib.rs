pub mod game {
    pub mod state;
    pub use state::*;

    pub mod board;
    pub use board::*;

    mod algorithm;

    pub mod parse;
    pub use parse::*;

    pub mod piece;
    pub use piece::*;

    pub mod player;
    pub use player::*;
}
