use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub size: usize,
    pub grid: Vec<Vec<Option<u32>>>,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let grid = vec![vec![None; size]; size];
        let mut game = Game {
            size,
            grid,
            score: 0,
            game_over: false,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }

    pub fn spawn_tile(&mut self) {
        let empty: Vec<(usize, usize)> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &cell)| {
                    if cell.is_none() {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if empty.is_empty() {
            self.game_over = true;
            return;
        }

        let idx = rand::thread_rng().gen_range(0..empty.len());
        let (i, j) = empty[idx];
        let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
        self.grid[i][j] = Some(value);
    }

    pub fn move_left(&mut self) -> bool {
        let mut moved = false;
        for row in self.grid.iter_mut() {
            let original = row.clone();
            let new_row = Self::merge(row.iter().filter_map(|&x| x).collect::<Vec<_>>());
            for i in 0..self.size {
                if i < new_row.len() {
                    row[i] = Some(new_row[i]);
                } else {
                    row[i] = None;
                }
            }
            if row != &original {
                moved = true;
            }
        }
        if moved {
            self.score += 1; // simplified scoring
            self.spawn_tile();
        }
        moved
    }

    pub fn move_right(&mut self) -> bool {
        self.reverse_grid();
        let moved = self.move_left();
        self.reverse_grid();
        moved
    }

    pub fn move_up(&mut self) -> bool {
        self.transpose_grid();
        let moved = self.move_left();
        self.transpose_grid();
        moved
    }

    pub fn move_down(&mut self) -> bool {
        self.transpose_grid();
        self.reverse_grid();
        let moved = self.move_left();
        self.reverse_grid();
        self.transpose_grid();
        moved
    }

    fn merge(mut tiles: Vec<u32>) -> Vec<u32> {
        let mut result = Vec::new();
        let mut skip = false;
        for i in 0..tiles.len() {
            if skip {
                skip = false;
                continue;
            }
            if i + 1 < tiles.len() && tiles[i] == tiles[i + 1] {
                result.push(tiles[i] * 2);
                skip = true;
            } else {
                result.push(tiles[i]);
            }
        }
        result
    }

    fn reverse_grid(&mut self) {
        for row in self.grid.iter_mut() {
            row.reverse();
        }
    }

    fn transpose_grid(&mut self) {
        let mut transposed = vec![vec![None; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                transposed[j][i] = self.grid[i][j];
            }
        }
        self.grid = transposed;
    }
}