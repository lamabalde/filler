use crate::game::{dimensions, instruction, Dimensions};

use super::Coordinates;

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub dimensions: Dimensions,
    pub anfield: Vec<Vec<char>>,
}

impl Board {
    pub fn new() -> Self {
        let dimensions = dimensions(&instruction());
        let mut board = Self {
            dimensions,
            anfield: Vec::with_capacity(dimensions.1 as usize),
        };

        for _ in 0..=dimensions.1 as usize {
            let instruction = &instruction();
            if instruction.contains(" .")
                || instruction.contains(" @")
                || instruction.contains(" $")
                || instruction.contains(" a")
                || instruction.contains(" s")
            {
                board.anfield(instruction);
            }
        }
        board
    }

    pub fn anfield(&mut self, s: &str) {
        let row = s.split_whitespace().next_back().unwrap().trim_end();
        self.anfield.push(row.chars().collect())
    }

    pub fn all_coords(
        &self,
    ) -> (
        Vec<Coordinates>,
        Vec<Coordinates>,
        Vec<Coordinates>,
        Vec<Coordinates>,
    ) {
        let mut p1_coords = Vec::new();
        let mut p2_coords = Vec::new();
        let mut playable_1 = Vec::new();
        let mut playable_2 = Vec::new();
        for (y, row) in self.anfield.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c.eq(&'@') || c.eq(&'a') {
                    let c = Coordinates::new(x as isize, y as isize);
                    if self.empty_neighbor(&c) {
                        playable_1.push(c.clone());
                    }
                    p1_coords.push(c);
                }
                if c.eq(&'$') || c.eq(&'s') {
                    let c = Coordinates::new(x as isize, y as isize);
                    if self.empty_neighbor(&c) {
                        playable_2.push(c.clone());
                    }
                    p2_coords.push(c);
                }
            }
        }

        (p1_coords, p2_coords, playable_1, playable_2)
    }

    pub fn empty_neighbor(&self, c: &Coordinates) -> bool {
        let x = c.x as usize;
        let y = c.y as usize;

        let rows = if y == 0 {
            self.anfield.iter().skip(0).take(2)
        } else {
            self.anfield.iter().skip(y - 1).take(3)
        };

        for row in rows {
            if x == 0 {
                if row.iter().take(2).any(|c| c == &'.') {
                    return true;
                }
            } else if row.iter().skip(x - 1).take(3).any(|c| c == &'.') {
                return true;
            }
        }
        false
    }

    pub fn last_piece(&self, player: u8) -> Vec<Coordinates> {
        let mut last_piece = Vec::new();
        if player == 1 {
            for (y, row) in self.anfield.iter().enumerate() {
                for (x, ch) in row.iter().enumerate() {
                    if ch.eq(&'s') {
                        last_piece.push(Coordinates::new(x as isize, y as isize));
                    }
                }
            }
        } else {
            for (y, row) in self.anfield.iter().enumerate() {
                for (x, ch) in row.iter().enumerate() {
                    if ch.eq(&'a') {
                        last_piece.push(Coordinates::new(x as isize, y as isize));
                    }
                }
            }
        }
        last_piece
    }

    pub fn width(&self) -> isize {
        self.dimensions.0
    }
    pub fn height(&self) -> isize {
        self.dimensions.1
    }
}
