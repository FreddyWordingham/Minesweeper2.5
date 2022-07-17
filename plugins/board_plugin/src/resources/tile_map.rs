use ndarray::Array2;

use crate::{components::Coordinates, resources::tile::Tile};

/// Delta coordinates for all 8 square neighbors
/// [6] [7] [8]
/// [4]     [5]
/// [1] [2] [3]
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
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

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map [{}x{}] with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> u16 {
        self.map.shape()[0] as u16
    }

    pub fn height(&self) -> u16 {
        self.map.shape()[1] as u16
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width() || coordinates.y >= self.height() {
            return false;
        };
        self.map[(coordinates.y as usize, coordinates.x as usize)].is_bomb()
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
