use sfml::{
    cpp::FBox,
    graphics::{
        Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View,
    },
    system::{Vector2f, Vector2i},
    window::{ContextSettings, Event, Key, Style, mouse::Button},
};

use crate::morpion::{Morpion, PlacementError, Team};

pub struct Handler<'a> {
    win: FBox<RenderWindow>,
    mpn: Morpion,
    grid: [RectangleShape<'a>; 9],
    ended: bool,
    background: Color,
}

impl<'a> Handler<'a> {
    pub fn new() -> Self {
        let mut win = RenderWindow::new(
            (800, 800),
            "Morpion",
            Style::default(),
            &ContextSettings::default(),
        )
        .unwrap();

        win.set_framerate_limit(240);

        let grid: [RectangleShape<'a>; 9] = core::array::from_fn(|i| {
            let mut cell = RectangleShape::with_size(Vector2f::new(200.0, 200.0));
            cell.set_position((50.0 + 250.0 * (i % 3) as f32, 50.0 + 250.0 * (i / 3) as f32));
            cell.set_fill_color(Color::BLACK);
            cell
        });

        Handler {
            win,
            grid,
            mpn: Morpion::new(),
            ended: false,
            background: Color::WHITE,
        }
    }

    pub fn gameloop(&mut self) {
        while self.win.is_open() {
            self.handle_events();
            self.render_everything();
        }
    }

    fn render_everything(&mut self) {
        self.win.clear(self.background);
        for cell in &self.grid {
            self.win.draw(cell);
        }
        self.win.display();
    }

    fn handle_events(&mut self) {
        while let Some(ev) = self.win.poll_event() {
            match ev {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => self.win.close(),
                Event::Resized { width, height } => self.resize(width, height),
                Event::MouseButtonPressed {
                    button: Button::Left,
                    x,
                    y,
                } => self.play_at(Vector2f::new(x as f32, y as f32)),
                _ => (),
            }
        }
    }

    fn play_at(&mut self, point: Vector2f) {
        if !self.human_plays_at(point) || self.ended {
            return;
        }

        self.check_win();
        if self.ended {
            self.update_grid();
            return;
        }

        self.mpn.bot_plays();
        self.check_win();
        self.update_grid();
    }

    /// Returns true if a play was made and false if not
    fn human_plays_at(&mut self, point: Vector2f) -> bool {
        for (i, cell) in self.grid.iter().enumerate() {
            if cell.global_bounds().contains(point) {
                match self.mpn.play_at(i, Team::Circle) {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }
        return false;
    }

    fn check_win(&mut self) {
        self.ended = true;
        match self.mpn.check_win() {
            Some(Team::Circle) => self.background = Color::rgb(0, 0, 64),
            Some(Team::Cross) => self.background = Color::rgb(64, 0, 0),
            Some(Team::Blank) => self.background = Color::rgb(64, 64, 64),
            _ => self.ended = false,
        }
    }

    fn update_grid(&mut self) {
        for (graphic_cell, logic_cell) in self.grid.iter_mut().zip(self.mpn.grid) {
            match logic_cell {
                Team::Circle => graphic_cell.set_fill_color(Color::BLUE),
                Team::Cross => graphic_cell.set_fill_color(Color::RED),
                Team::Blank => graphic_cell.set_fill_color(Color::BLACK),
            }
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.win
            .set_view(&*View::from_rect(Rect::new(0., 0., width as f32, height as f32)).unwrap());
    }
}
