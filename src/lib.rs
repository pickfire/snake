use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use termion::color;

pub mod client;
pub mod server;

/// Game state passed arough through network.
#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    snakes: Vec<Snake>,
}

/// Point.
pub type Point = (u8, u8);

// Direction.
#[derive(Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Color.
#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn to_term_color(&self) -> &'static str {
        match self {
            Color::Red => color::Red.fg_str(),
            Color::Green => color::Green.fg_str(),
            Color::Blue => color::Blue.fg_str(),
        }
    }
}

/// Snake.
#[derive(Serialize, Deserialize)]
pub struct Snake {
    color: Color,
    direction: Direction,
    body: VecDeque<Point>,
    /// Turns for the snake to grow.
    shed_in: usize,
}
