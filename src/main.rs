use std::io::Write;

struct Morpion {
    grid: [Option<bool>; 9],
    turn: bool,
}

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
                    return idx-1;
                }
            }
            Err(_) => continue,
        }
    }
}

impl Morpion {
    fn new() -> Self {
        Morpion {
            grid: [None; 9],
            turn: false,
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

    fn check_win(&self) -> Option<Option<bool>> {
        for pattern in WIN_PATTERN {
            if self.grid[pattern[0]] == self.grid[pattern[1]]
                && self.grid[pattern[1]] == self.grid[pattern[2]]
                && self.grid[pattern[0]].is_some()
            {
                return Some(self.grid[pattern[0]]);
            }
        }

        if self.grid.iter().all(|c| c.is_some()) {
            Some(None)
        } else {
            None
        }
    }

    fn gameloop(&mut self) {
        loop {
            println!("{}", self);

            if let Some(end_entity) = self.check_win() {
                match end_entity {
                    Some(player) => println!("Player {} wins !", if player { 'O' } else { 'X' }),
                    None => println!("Égalité !"),
                }
                return;
            }

            loop {
                match self.play_at(get_user_input()) {
                    Ok(()) => (),
                    Err(_) => continue,
                }
                break
            }

            self.turn = !self.turn
        }
    }
}

fn main() {
    let mut mpn = Morpion::new();
    mpn.gameloop();
}
