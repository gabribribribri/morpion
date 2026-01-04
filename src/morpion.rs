
pub struct Morpion {
    grid: [Team; 9],
    turn: Team,
}

#[derive(Debug)]
enum PlacementError {
    OutOfRange,
    CellAlreadyOccupied,
}

#[derive(Copy, Clone, PartialEq)]
enum Team {
    Circle,
    Cross,
    Blank
}

use Team::*;

impl std::ops::Not for Team {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Circle => Cross,
            Cross => Circle,
            Blank => panic!("Cannot negate empty")
        }
    }
}


const WIN_PATTERN: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl Morpion {
    pub fn new() -> Self {
        Morpion {
            grid: [Blank; 9],
            turn: Circle,
        }
    }

    fn play_at(&mut self, idx: usize) -> Result<(), PlacementError> {
        match self.grid.get(idx) {
            None => Err(PlacementError::OutOfRange),
            Some(cell) => match cell {
                Circle | Cross => Err(PlacementError::CellAlreadyOccupied),
                Blank => {
                    self.grid[idx] = self.turn;
                    Ok(())
                }
            },
        }
    }

    pub fn gameloop(&mut self) {
        loop {
            if self.turn == Circle {
                println!("{}", self);
            }

            if let Some(end_entity) = check_win(self.grid) {
                match end_entity {
                    Blank => println!("Égalité !"),
                    team => println!("Player {} wins !", in_pion(team)),
                }
                return;
            }

            match self.turn {
                Circle => self.human_plays(),
                Cross => self.bot_plays(),
                Blank => panic!("Blank is not supposed to play")
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
                next_grid[i] = Cross;
                let wp = win_proba(next_grid, Circle);
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

fn check_win(grid: [Team; 9]) -> Option<Team> {
    for pattern in WIN_PATTERN {
        if grid[pattern[0]] == grid[pattern[1]]
            && grid[pattern[1]] == grid[pattern[2]]
            && grid[pattern[0]] != Blank
        {
            return Some(grid[pattern[0]]);
        }
    }

    if grid.iter().all(|c| *c != Blank ) {
        Some(Blank)
    } else {
        None
    }
}

fn indexes_empty_cell(grid: [Team; 9]) -> impl std::iter::Iterator<Item = usize> {
    grid.into_iter()
        .enumerate()
        .filter(|(_, c)| *c == Blank)
        .map(|(i, _)| i)
}

fn win_proba(grid: [Team; 9], turn: Team) -> f32 {
    match check_win(grid) {
        Some(Circle) => return 0.0,
        Some(Cross) => return 1.0,
        Some(Blank) => return 0.5,
        None => (),
    }

    let mut number_of_possible_move = 0.0;
    let mut win_probability = 0.0;
    for i in indexes_empty_cell(grid) {
        let mut next_grid = grid;
        next_grid[i] = turn;
        win_probability += win_proba(next_grid, !turn);
        number_of_possible_move += 1.0
    }

    return win_probability / number_of_possible_move;
}



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
        write!(f, "{} >>", in_pion(self.turn))?;
        Ok(())
    }
}

fn in_pion(c: Team) -> char {
    match c {
        Blank => ' ',
        Circle => 'O',
        Cross => 'X',
    }
}
