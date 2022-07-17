use ndarray::Array2;

use crate::resources::tile::Tile;

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
}
