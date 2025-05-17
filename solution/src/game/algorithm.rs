use crate::game::{Coordinates, Piece, State};

type Distance = (isize, Coordinates);

impl State {
    pub fn make_move(&mut self, piece: &Piece) {
        let self_coords = &self.get_playable_coords().0;
        if self.endgame {
            if self.end_game() {
                println!("0 0");
                return;
            }
            for c in self_coords {
                if let Some(placement_coords) = self.can_place(c, piece) {
                    println!("{}", placement_coords[0]);
                    return;
                }
            }
            println!("0 0");
            return;
        }
        if let Some(block_coords) = self.block() {
            if self.place_piece(&block_coords, piece) {
                return;
            }
        }

        if !self.place_piece(self_coords, piece) {
            println!("0 0");
        }
    }

    fn place_piece(&mut self, coords: &[Coordinates], piece: &Piece) -> bool {
        for c in &self.sort_distances(coords) {
            if let Some(placement_coords) = self.can_place(c, piece) {
                let placement = self.shortest_distance(&placement_coords);
                self.insert(&placement, piece);
                println!("{}", placement);
                return true;
            }
        }
        false
    }

    fn sort_distances(&self, self_coords: &[Coordinates]) -> Vec<Coordinates> {
        let other_coords = self.get_playable_coords().1;
        let mut distances: Vec<Distance> = Vec::with_capacity(self_coords.len());
        for c1 in self_coords.iter() {
            let mut dist = isize::MAX;
            for c2 in &other_coords {
                let current_dist = c1.calc_dist(c2);
                if current_dist < dist {
                    dist = current_dist;
                }
            }
            distances.push((dist, c1.clone()));
        }
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        distances.into_iter().map(|d| d.1).collect()
    }

    fn can_place(&self, c: &Coordinates, p: &Piece) -> Option<Vec<Coordinates>> {
        let (self_coords, other_coords) = self.get_player_coords();

        let mut placeable = Vec::new();
        let borders = p.borders();

        for overlap in &borders {
            let mut allowed_to_place = true;
            for piece_coords in borders.iter().filter(|&coord| coord != overlap) {
                let x = c.x + piece_coords.x - overlap.x;
                let y = c.y + piece_coords.y - overlap.y;

                let piece_coordinate = Coordinates::new(x, y);
                if self.out_of_bounds(&piece_coordinate)
                    || other_coords.contains(&piece_coordinate)
                    || self_coords.contains(&piece_coordinate)
                {
                    allowed_to_place = false;
                    break;
                }
            }
            if allowed_to_place {
                placeable.push(Coordinates::new(c.x - overlap.x, c.y - overlap.y));
            }
        }

        if placeable.is_empty() {
            None
        } else {
            Some(placeable)
        }
    }

    fn block(&self) -> Option<Vec<Coordinates>> {
        let (self_coords, other_coords) = self.get_playable_coords();
        let (self_left, self_right, self_top, self_bottom) = self.border_coords(&self_coords);
        let (other_left, other_right, other_top, other_bottom) = self.border_coords(&other_coords);

        let (width, height) = self.board.dimensions;

        if self_left < other_left
            && self_right > other_right
            && self_top < other_top
            && self_bottom > other_bottom
        {
            return None;
        }

        if self_left == 0 && self_right == width - 1 && self_top == 0 && self_bottom == height - 1 {
            return None;
        }

        let mut block_coords = Vec::new();

        let collect_x = |x: isize| -> Vec<Coordinates> {
            self_coords.iter().filter(|&c| c.x == x).cloned().collect()
        };

        let collect_y = |y: isize| -> Vec<Coordinates> {
            self_coords.iter().filter(|&c| c.y == y).cloned().collect()
        };

        if self_left > other_left {
            let left_coords = collect_x(self_left);
            if !left_coords.is_empty() {
                block_coords.push(left_coords);
            }
        }

        if self_right < other_right {
            let right_coords = collect_x(self_right);
            if !right_coords.is_empty() {
                block_coords.push(right_coords);
            }
        }

        if self_top > other_top {
            let top_coords = collect_y(self_top);
            if !top_coords.is_empty() {
                block_coords.push(top_coords);
            }
        }

        if self_bottom < other_bottom {
            let bottom_coords = collect_y(self_bottom);
            if !bottom_coords.is_empty() {
                block_coords.push(bottom_coords);
            }
        }

        if block_coords.is_empty() {
            None
        } else {
            Some(block_coords.into_iter().flatten().collect())
        }
    }

    fn shortest_distance(&self, coords: &[Coordinates]) -> Coordinates {
        let other_coords = if self.player == 1 {
            &self.p2.playable
        } else {
            &self.p1.playable
        };

        let mut shortest_distance = isize::MAX;
        let mut closest_coord = Coordinates::default();

        for coord in other_coords {
            for piece_coord in coords {
                let distance = coord.calc_dist(piece_coord);

                if distance < shortest_distance {
                    shortest_distance = distance;
                    closest_coord = piece_coord.clone();
                }
            }
        }
        closest_coord
    }

    fn end_game(&self) -> bool {
        if self.player == 1 {
            self.score.0 > self.score.1
        } else {
            self.score.1 > self.score.0
        }
    }

    fn out_of_bounds(&self, coord: &Coordinates) -> bool {
        let (width, height) = self.board.dimensions;
        coord.x < 0 || coord.x >= width || coord.y < 0 || coord.y >= height
    }
    pub fn placeable(&self, c: &Coordinates, piece: &Piece) -> bool {
        let mut overlapping_self = 0;
        let (self_coords, other_coords) = self.get_player_coords();

        for coord in piece.borders() {
            let x = c.x + coord.x;
            let y = c.y + coord.y;
            if x < 0 || y < 0 {
                continue;
            }
            let placement = Coordinates::new(x, y);
            overlapping_self += self_coords
                .iter()
                .filter(|&placed| placed.eq(&placement))
                .count();

            if other_coords.iter().any(|placed| placed.eq(&placement)) {
                return false;
            }

            if overlapping_self > 1 {
                return false;
            }
        }
        overlapping_self == 1
    }

    fn get_player_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.coords.clone(), self.p2.coords.clone())
        } else {
            (self.p2.coords.clone(), self.p1.coords.clone())
        }
    }

    fn get_playable_coords(&self) -> (Vec<Coordinates>, Vec<Coordinates>) {
        if self.player == 1 {
            (self.p1.playable.clone(), self.p2.playable.clone())
        } else {
            (self.p2.playable.clone(), self.p1.playable.clone())
        }
    }
    fn border_coords(&self, coords: &[Coordinates]) -> (isize, isize, isize, isize) {
        (
            self.left_coord(coords),
            self.right_coord(coords),
            self.top_coord(coords),
            self.bottom_coord(coords),
        )
    }
    fn top_coord(&self, coords: &[Coordinates]) -> isize {
        let mut y = self.board.dimensions.1;
        for c in coords {
            if c.y < y {
                y = c.y;
            }
        }
        y
    }
    fn bottom_coord(&self, coords: &[Coordinates]) -> isize {
        let mut y = 0;
        for c in coords {
            if c.y > y {
                y = c.y;
            }
        }
        y
    }
    fn left_coord(&self, coords: &[Coordinates]) -> isize {
        let mut x = self.board.dimensions.0;
        for c in coords {
            if c.x < x {
                x = c.x;
            }
        }
        x
    }
    fn right_coord(&self, coords: &[Coordinates]) -> isize {
        let mut x = 0;
        for c in coords {
            if c.x > x {
                x = c.x;
            }
        }
        x
    }
}
