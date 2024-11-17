use crate::game::Game;

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn get_possible_moves(&self) -> Vec<Direction> {
        let mut moves = Vec::new();
        let mut game_copy = self.clone();

        if game_copy.move_left() {
            moves.push(Direction::Left);
        }

        game_copy = self.clone();
        if game_copy.move_right() {
            moves.push(Direction::Right);
        }

        game_copy = self.clone();
        if game_copy.move_up() {
            moves.push(Direction::Up);
        }

        game_copy = self.clone();
        if game_copy.move_down() {
            moves.push(Direction::Down);
        }

        moves
    }

    pub fn ai_make_move(&mut self) -> Option<Direction> {
        let possible = self.get_possible_moves();
        if possible.is_empty() {
            self.game_over = true;
            return None;
        }

        let mut best_move: Option<(Direction, u32)> = None;

        for direction in possible {
            let mut game_copy = self.clone();
            let moved = match &direction {
                Direction::Left => game_copy.move_left(),
                Direction::Right => game_copy.move_right(),
                Direction::Up => game_copy.move_up(),
                Direction::Down => game_copy.move_down(),
            };

            if moved {
                if let Some((_, best_score)) = &best_move {
                    if game_copy.score > *best_score {
                        best_move = Some((direction.clone(), game_copy.score));
                    }
                } else {
                    best_move = Some((direction.clone(), game_copy.score));
                }
            }
        }

        best_move.map(|(dir, _)| dir)
    }
}