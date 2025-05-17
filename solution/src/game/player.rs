use crate::game::{Board, Coordinates};

#[derive(Default, Debug)]
pub struct Player {
    pub coords: Vec<Coordinates>,
    pub playable: Vec<Coordinates>,
}

impl Player {
    pub fn new(coords: Vec<Coordinates>, playable: Vec<Coordinates>) -> Self {
        Self { coords, playable }
    }

    pub fn init(board: &Board) -> (Self, Self) {
        let (p1_coords, p2_coords, playable_1, playable_2) = board.all_coords();
        (
            Player::new(p1_coords, playable_1),
            Player::new(p2_coords, playable_2),
        )
    }

    pub fn top_y(&self) -> isize {
        self.coords[0].y
    }

    pub fn bottom_y(&self) -> isize {
        self.coords.last().unwrap().y
    }

    pub fn left_x(&self) -> isize {
        let mut left = 999;
        for coordinates in &self.coords {
            if coordinates.x < left {
                left = coordinates.x;
            }
        }
        left
    }

    pub fn right_x(&self) -> isize {
        let mut right = 0;
        for coordinates in &self.coords {
            if coordinates.x > right {
                right = coordinates.x;
            }
        }
        right
    }
}
