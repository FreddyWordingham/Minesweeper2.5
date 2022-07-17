use nalgebra::Vector2;
use ndarray::Array2;
use rand::{thread_rng, Rng};

use crate::{components::Coordinates, resources::Tile};

/// Delta coordinates for all 8 square neighbors
/// [6] [7] [8]
/// [4]     [5]
/// [1] [2] [3]
const SQUARE_COORDINATES: [Vector2<i8>; 8] = [
    Vector2::<i8>::new(-1, -1),
    Vector2::<i8>::new(0, -1),
    Vector2::<i8>::new(1, -1),
    Vector2::<i8>::new(-1, 0),
    Vector2::<i8>::new(1, 0),
    Vector2::<i8>::new(-1, 1),
    Vector2::<i8>::new(0, 1),
    Vector2::<i8>::new(1, 1),
];

/// Base tile map.
#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    map: Array2<Tile>,
}

impl TileMap {
    /// Generate an empty map.
    pub fn empty(width: u16, height: u16) -> Self {
        let map = Array2::from_elem((width as usize, height as usize), Tile::Empty);
        Self { bomb_count: 0, map }
    }

    /// Places bombs and bomb neighbor tiles.
    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width()) as usize,
                rng.gen_range(0..self.height()) as usize,
            );
            if let Tile::Empty = self.map[(x, y)] {
                self.map[(x, y)] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        // Place bomb neighbors
        for y in 0..self.height() {
            for x in 0..self.width() {
                let coords = Coordinates::new(x as i8, y as i8);
                if self.is_bomb_at(coords) {
                    continue;
                }
                let num = self.bomb_count_at(coords);
                if num == 0 {
                    continue;
                }
                self.map[(x as usize, y as usize)] = Tile::BombNeighbor(num);
            }
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map [{}x{}] with {} bombs:\n",
            self.width(),
            self.height(),
            self.bomb_count
        );
        let line: String = (0..=(self.width() + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);

        for y in 0..self.height() {
            buffer = format!("{}|", buffer);
            for x in 0..self.width() {
                buffer = format!(
                    "{}{}",
                    buffer,
                    self.map[(x as usize, y as usize)].console_output()
                );
            }
            buffer = format!("{}|\n", buffer);
        }

        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> usize {
        self.map.shape()[0]
    }

    pub fn height(&self) -> usize {
        self.map.shape()[1]
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .map(move |offset| coordinates + offset)
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x as usize >= self.width() || coordinates.y as usize >= self.height() {
            return false;
        };
        self.map[(coordinates.x as usize, coordinates.y as usize)].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        self.safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count() as u8
    }
}
