use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use termion::{color};

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
    pub fn to_term_color(&self) -> Box<dyn color::Color> {
        match self {
            Color::Red => Box::new(color::Red),
            Color::Green => Box::new(color::Green),
            Color::Blue => Box::new(color::Blue),
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
