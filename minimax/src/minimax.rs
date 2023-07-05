pub const P1: i8 = -1;
pub const AI: i8 = 1;

#[derive(Debug)]
pub struct Moves {
    pub x: Option<usize>,
    pub y: Option<usize>,
}

#[derive(Debug)]
pub struct Minimax {
    pub moves: Moves,
    pub score: isize,
}

pub type Board = [[i8; 3]; 3];

pub trait BoardOps {
    fn new() -> Self;
    fn optimal_move(&mut self, depth: usize, player: i8) -> Minimax;
    fn empty_cells(&self) -> Vec<Moves>;
    fn depth(&self) -> usize;
    fn wins(&self, player: i8) -> bool;
    fn game_over(&self) -> bool;
    fn evaluate(&self) -> isize;
}

impl BoardOps for Board {
    fn new() -> Self {
        return [[0; 3]; 3];
    }

    fn evaluate(&self) -> isize {
        let score = if self.wins(P1) {
            -1
        } else if self.wins(AI) {
            1
        } else {
            0
        };
        return score;
    }

    fn wins(&self, player: i8) -> bool {
        let win_state = [
            [self[0][0], self[0][1], self[0][2]],
            [self[1][0], self[1][1], self[1][2]],
            [self[2][0], self[2][1], self[2][2]],
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
            [self[0][0], self[1][1], self[2][2]],
            [self[2][0], self[1][1], self[0][2]],
        ];

        return win_state.contains(&[player, player, player]);
    }

    fn game_over(&self) -> bool {
        return self.wins(P1) || self.wins(AI);
    }

    fn empty_cells(&self) -> Vec<Moves> {
        let mut cells = Vec::new();

        for (x, row) in self.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    cells.push(Moves {
                        x: Some(x),
                        y: Some(y),
                    });
                }
            }
        }
        return cells;
    }

    fn depth(&self) -> usize {
        return self.empty_cells().len();
    }

    fn optimal_move(&mut self, depth: usize, player: i8) -> Minimax {
        let mut best: Minimax;
        if player == AI {
            best = Minimax {
                moves: Moves {
                    x: Some(0),
                    y: Some(0),
                },
                score: std::isize::MIN,
            }
        } else {
            best = Minimax {
                moves: Moves {
                    x: Some(0),
                    y: Some(0),
                },
                score: std::isize::MAX,
            }
        }

        if depth == 0 || self.game_over() {
            return Minimax {
                moves: Moves {
                    x: Some(0),
                    y: Some(0),
                },
                score: self.evaluate(),
            };
        }

        for cell in self.empty_cells() {
            if let Moves {
                x: Some(x),
                y: Some(y),
            } = cell
            {
                self[x][y] = player;
                let mut minimax = self.optimal_move(depth - 1, -player); // alternates player
                self[x][y] = 0;

                minimax.moves = Moves {
                    x: Some(x),
                    y: Some(y),
                };

                if player == AI {
                    if minimax.score > best.score {
                        best = minimax;
                    }
                } else {
                    if minimax.score < best.score {
                        best = minimax;
                    }
                }
            }
        }

        return best;
    }
}
