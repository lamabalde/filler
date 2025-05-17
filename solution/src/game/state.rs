use super::{instruction, Board, Piece};
use crate::game::Player;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct State {
    pub board: Board,
    pub score: (u64, u64),
    pub round: u64,
    pub p1: Player,
    pub p2: Player,
    pub player: u8,
    pub endgame: bool,
}

impl State {
    pub fn new(board: Board, player: u8, players: (Player, Player)) -> Self {
        Self {
            board,
            score: (1, 1),
            round: 1,
            p1: players.0,
            p2: players.1,
            player,
            endgame: false,
        }
    }

    pub fn update(&mut self) {
        for row in self.board.anfield.iter_mut() {
            if row.contains(&'a') || row.contains(&'s') {
                for cell in row.iter_mut() {
                    if cell == &mut 'a' {
                        *cell = '@';
                    } else if cell == &mut 's' {
                        *cell = '$';
                    }
                }
            }
        }
        loop {
            let instruction = instruction();
            if instruction.contains("   0") {
                break;
            }
        }
        for y in 0..self.board.dimensions.1 as usize {
            let row = instruction()
                .split_ascii_whitespace()
                .skip(1)
                .collect::<String>();

            if row.contains('a') || row.contains('s') {
                for (x, cell) in row.chars().enumerate() {
                    if cell == 'a' {
                        self.board.anfield[y][x] = 'a';
                    } else if cell == 's' {
                        self.board.anfield[y][x] = 's';
                    }
                }
            }
        }
        let (p1_score, p2_score) = (self.p1.coords.len(), self.p2.coords.len());
        let (p1, p2) = Player::init(&self.board);
        self.p1 = p1;
        self.p2 = p2;

        let (new_p1_score, new_p2_score) = (self.p1.coords.len(), self.p2.coords.len());

        if !self.endgame
            && (self.player == 1 && new_p2_score == p2_score
                || self.player == 2 && new_p1_score == p1_score)
        {
            self.endgame = true;
        }

        self.score = (new_p1_score as u64, new_p2_score as u64);
        self.round += 1;
    }

    pub fn insert(&mut self, c: &Coordinates, piece: &Piece) {
        let character = if self.player == 1 { 'a' } else { 's' };

        for piece_c in piece.borders() {
            let x = (piece_c.x + c.x) as usize;
            let y = (piece_c.y + c.y) as usize;
            self.board.anfield[y][x] = character;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl Coordinates {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn calc_dist(&self, other: &Coordinates) -> isize {
        if (self.x - other.x).abs() >= (self.y - other.y).abs() {
            (self.x - other.x).abs() - 1
        } else {
            (self.y - other.y).abs() - 1
        }
    }
}
