use crate::game::{dimensions, instruction, Coordinates, Dimensions};

#[derive(Debug, Clone, Default)]
pub struct Piece {
    pub dimensions: Dimensions,
    pub shape: Vec<Vec<char>>,
}

impl Piece {
    pub fn new() -> Self {
        let dimensions = dimensions(&instruction());
        let mut piece = Self {
            dimensions,
            shape: Vec::with_capacity(dimensions.1 as usize),
        };
        for _ in 0..dimensions.1 as usize {
            let instruction = &instruction();
            piece.shape(instruction);
        }
        piece
    }

    pub fn shape(&mut self, s: &str) {
        self.shape.push(s.trim_end().chars().collect())
    }

    pub fn borders(&self) -> Vec<Coordinates> {
        let mut borders = Vec::new();
        for (y, row) in self.shape.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                if ch.eq(&'O') {
                    borders.push(Coordinates::new(x as isize, y as isize));
                }
            }
        }
        borders
    }

    pub fn width(&self) -> isize {
        self.right() - self.left() + 1
    }

    pub fn height(&self) -> isize {
        self.bottom() - self.top() + 1
    }

    pub fn top(&self) -> isize {
        let mut y = self.dimensions.1;
        for c in self.borders() {
            if c.y < y {
                y = c.y;
            }
        }
        y
    }

    pub fn bottom(&self) -> isize {
        let mut y = 0;
        for c in self.borders() {
            if c.y > y {
                y = c.y;
            }
        }
        y
    }

    pub fn left(&self) -> isize {
        let mut x = self.dimensions.0;
        for c in self.borders() {
            if c.x < x {
                x = c.x;
            }
        }
        x
    }

    pub fn right(&self) -> isize {
        let mut x = 0;
        for c in self.borders() {
            if c.x > x {
                x = c.x;
            }
        }
        x
    }

    pub fn wide(&self) -> bool {
        self.width() > self.height()
    }
}
