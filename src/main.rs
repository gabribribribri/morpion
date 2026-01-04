use std::io::Write;

struct Morpion {
    grid: [Option<bool>; 9],
    turn: bool,
}

#[derive(Debug)]
enum PlacementError {
    OutOfRange,
    CellAlreadyOccupied,
}

const WIN_PATTERN: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 5, 8],
];

impl std::fmt::Display for Morpion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.grid.chunks(3) {
            writeln!(
                f,
                "{} | {} | {}",
                in_pion(w[0]),
                in_pion(w[1]),
                in_pion(w[2])
            )?;
        }
        write!(f, "{} >>", in_pion(Some(self.turn)))?;
        Ok(())
    }
}

fn in_pion(c: Option<bool>) -> char {
    match c {
        None => ' ',
        Some(true) => 'O',
        Some(false) => 'X',
    }
}

fn get_user_input() -> usize {
    loop {
        let mut s = String::new();
        match std::io::stdin().read_line(&mut s) {
            Ok(_) => (),
            Err(_) => continue,
        }
        match s.trim().parse::<usize>() {
            Ok(idx) => {
                if idx == 0 || idx >= 10 {
                    continue;
                } else {
                    return idx - 1;
                }
            }
            Err(_) => continue,
        }
    }
}
fn check_win(grid: [Option<bool>; 9]) -> Option<Option<bool>> {
    for pattern in WIN_PATTERN {
        if grid[pattern[0]] == grid[pattern[1]]
            && grid[pattern[1]] == grid[pattern[2]]
            && grid[pattern[0]].is_some()
        {
            return Some(grid[pattern[0]]);
        }
    }

    if grid.iter().all(|c| c.is_some()) {
        Some(None)
    } else {
        None
    }
}

fn indexes_empty_cell(grid: [Option<bool>; 9]) -> impl std::iter::Iterator<Item = usize> {
    grid.into_iter()
        .enumerate()
        .filter(|(_, c)| c.is_none())
        .map(|(i, _)| i)
}
impl Morpion {
    fn new() -> Self {
        Morpion {
            grid: [None; 9],
            turn: true,
        }
    }

    fn play_at(&mut self, idx: usize) -> Result<(), PlacementError> {
        match self.grid.get(idx) {
            None => Err(PlacementError::OutOfRange),
            Some(cell) => match cell {
                Some(_) => Err(PlacementError::CellAlreadyOccupied),
                None => {
                    self.grid[idx] = Some(self.turn);
                    Ok(())
                }
            },
        }
    }

    fn gameloop(&mut self) {
        loop {
            if self.turn {
                println!("{}", self);
            }

            if let Some(end_entity) = check_win(self.grid) {
                match end_entity {
                    Some(player) => println!("Player {} wins !", if player { 'O' } else { 'X' }),
                    None => println!("Égalité !"),
                }
                return;
            }

            match self.turn {
                true => self.human_plays(),
                false => self.bot_plays(),
            }

            self.turn = !self.turn;
        }
    }

    fn human_plays(&mut self) {
        loop {
            match self.play_at(get_user_input()) {
                Ok(()) => (),
                Err(_) => continue,
            }
            break;
        }
    }

    fn bot_plays(&mut self) {
        let idx_to_play = indexes_empty_cell(self.grid)
            .map(|i| {
                let mut next_grid = self.grid;
                next_grid[i] = Some(false);
                let wp = win_proba(next_grid, false);
                println!(">> i={} -> p={}", i, wp);
                (i, wp)
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap()
            .0;
        println!(">>> will play at {}", idx_to_play);
        self.play_at(idx_to_play).unwrap()
    }
}

fn main() {
    let mut mpn = Morpion::new();
    mpn.gameloop();
}

fn win_proba(grid: [Option<bool>; 9], turn: bool) -> f32 {
    match check_win(grid) {
        Some(Some(true)) | Some(None) => return 0.0,
        Some(Some(false)) => return 1.0,
        None => (),
    }

    let mut number_of_possible_move = 0.0;
    let mut win_probability = 0.0;
    for i in indexes_empty_cell(grid) {
        let mut next_grid = grid;
        next_grid[i] = Some(turn);
        win_probability += win_proba(next_grid, !turn);
        number_of_possible_move += 1.0
    }

    return win_probability / number_of_possible_move;
}
