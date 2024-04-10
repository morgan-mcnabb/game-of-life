use crate::display_driver::Pixel;
#[derive(Debug)]
pub struct Life {
    pub board: Vec<Vec<Pixel>>,
    scale: usize,
}

impl Life {
    pub fn build(window_width: usize, window_height: usize, scale: usize) -> Result<Life, String> {
        if window_width % scale != 0 || window_height % scale != 0 {
            return Err(format!(
                "scale is not compatible with window width or window height"
            ));
        }

        let x_squares_per_row = window_width / scale;
        let y_squares_per_row = window_height / scale;

        let mut board: Vec<Vec<Pixel>> = Vec::new();
        for y in 0..y_squares_per_row {
            let mut row: Vec<Pixel> = Vec::new();
            for x in 0..x_squares_per_row {
                let pixel = Pixel::new(x, y, false);
                row.push(pixel);
            }
            board.push(row);
        }

        Ok(Life { board, scale })
    }

    pub fn glider(&mut self) {
        self.clear_board();
        self.board[1][0].turn_on();
        self.board[1][2].turn_on();
        self.board[0][2].turn_on();
        self.board[2][2].turn_on();
        self.board[2][1].turn_on();
    }

    pub fn pulsar(&mut self) {
        self.clear_board();
        self.board[3][40].turn_on();
        self.board[3][41].turn_on();
        self.board[3][42].turn_on();
        self.board[3][46].turn_on();
        self.board[3][47].turn_on();
        self.board[3][48].turn_on();
        self.board[5][38].turn_on();
        self.board[5][43].turn_on();
        self.board[5][45].turn_on();
        self.board[5][50].turn_on();
        self.board[6][38].turn_on();
        self.board[6][43].turn_on();
        self.board[6][45].turn_on();
        self.board[6][50].turn_on();
        self.board[7][38].turn_on();
        self.board[7][43].turn_on();
        self.board[7][45].turn_on();
        self.board[7][50].turn_on();
        self.board[8][40].turn_on();
        self.board[8][41].turn_on();
        self.board[8][42].turn_on();
        self.board[8][46].turn_on();
        self.board[8][47].turn_on();
        self.board[8][48].turn_on();
        self.board[10][40].turn_on();
        self.board[10][41].turn_on();
        self.board[10][42].turn_on();
        self.board[10][46].turn_on();
        self.board[10][47].turn_on();
        self.board[10][48].turn_on();
        self.board[11][38].turn_on();
        self.board[11][43].turn_on();
        self.board[11][45].turn_on();
        self.board[11][50].turn_on();
        self.board[12][38].turn_on();
        self.board[12][43].turn_on();
        self.board[12][45].turn_on();
        self.board[12][50].turn_on();
        self.board[13][38].turn_on();
        self.board[13][43].turn_on();
        self.board[13][45].turn_on();
        self.board[13][50].turn_on();
        self.board[15][40].turn_on();
        self.board[15][41].turn_on();
        self.board[15][42].turn_on();
        self.board[15][46].turn_on();
        self.board[15][47].turn_on();
        self.board[15][48].turn_on();
    }

    pub fn get_board(&mut self) -> &mut Vec<Vec<Pixel>> {
        &mut self.board
    }

    pub fn clear_board(&mut self) {
        self.board.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|pixel| pixel.is_on())
                .for_each(|pixel| pixel.kill());
        })
    }

    fn determine_next_generation(cloned_board: &Vec<Vec<Pixel>>, pixel: &mut Pixel) {
        let height = cloned_board.len();
        let width = cloned_board[0].len();
        let mut neighbor_pixels = Vec::new();

        let (x, y) = pixel.get_coords();

        let left_neighbor_x = (x + width - 1) % width;
        let right_neighbor_x = (x + 1) % width;
        let top_neighbor_y = (y + height - 1) % height;
        let bottom_neighbor_y = (y + 1) % height;

        neighbor_pixels.push(cloned_board[top_neighbor_y][left_neighbor_x]);
        neighbor_pixels.push(cloned_board[top_neighbor_y][x]);
        neighbor_pixels.push(cloned_board[top_neighbor_y][right_neighbor_x]);
        neighbor_pixels.push(cloned_board[y][left_neighbor_x]);
        neighbor_pixels.push(cloned_board[y][right_neighbor_x]);
        neighbor_pixels.push(cloned_board[bottom_neighbor_y][left_neighbor_x]);
        neighbor_pixels.push(cloned_board[bottom_neighbor_y][x]);
        neighbor_pixels.push(cloned_board[bottom_neighbor_y][right_neighbor_x]);

        let alive_neighbors = neighbor_pixels.iter().filter(|p| p.is_on()).count();
        if pixel.is_on() {
            if alive_neighbors < 2 || alive_neighbors > 3 {
                pixel.kill();
            } else {
                pixel.survive();
            }
        } else {
            if alive_neighbors == 3 {
                pixel.survive();
            }
        }
    }

    pub fn apply_rules(&mut self) {
        let cloned_board = self.board.clone();
        self.board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|pixel| {
                Life::determine_next_generation(&cloned_board, pixel);
            });
        });

        self.board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|pixel| {
                if pixel.is_alive_next_generation() {
                    pixel.turn_on();
                } else {
                    pixel.turn_off();
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_life() {
        let life = Life::build(100, 60, 10).unwrap();
        // num squares per column
        assert_eq!(6, life.board.len());
        //num swuares per row
        assert_eq!(10, life.board[0].len());
        assert_eq!(10, life.scale);
    }

    #[test]
    fn fail_to_create_life() {
        let life = Life::build(100, 60, 11);
        assert!(life.is_err());

        let err = life.unwrap_err();
        assert_eq!(
            "scale is not compatible with window width or window height",
            err
        );
    }
}
