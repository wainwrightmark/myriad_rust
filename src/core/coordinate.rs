use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(PartialEq, Debug, PartialOrd, Eq, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.row, self.column)
    }
}

impl Coordinate {
    pub fn rotate_and_flip<const C: usize, const R: usize>(
        &self,
        rotation: i8,
        flip: bool,
    ) -> Coordinate {
        let max = std::cmp::max(C, R) - 1;

        let rotated = match rotation.rem_euclid(4) {
            0 => *self,
            1 => Coordinate {
                row: max - self.column,
                column: self.row,
            },
            2 => Coordinate {
                row: max - self.row,
                column: max - self.column,
            },
            3 => Coordinate {
                row: self.column,
                column: max - self.row,
            },
            _ => panic!("Vaule should be 0, 1,2,3"),
        };

        if flip {
            return rotated.reflect_column::<C, R>();
        }

        rotated
    }

    pub fn get_transform<const C: usize, const R: usize>(
        &self,
        target: Coordinate,
    ) -> Option<(i8, bool)> {
        for flip in [false, true] {
            for rotation in [0, 1, 2, 3] {
                let r = self.rotate_and_flip::<C, R>(rotation, flip);

                if r == target {
                    return Some((rotation, flip));
                }
            }
        }

        None
    }

    pub fn reflect_column<const C: usize, const R: usize>(&self) -> Coordinate {
        Coordinate {
            row: self.row,
            column: C - 1 - self.column,
        }
    }

    pub fn get_angle(&self, other: Coordinate) -> f64 {
        let x_diff = other.column as f64 - self.column as f64;
        let y_diff = other.row as f64 - self.row as f64;

        (y_diff).atan2(x_diff).to_degrees()
    }

    ///True if two coordinates are orthogonal or diagonal
    pub fn is_adjacent(self, other: Coordinate) -> bool {
        //TODO dont use &
        if self == other {
            return false;
        };

        let row_diff = if self.row > other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };
        let col_diff = if self.column > other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };

        row_diff <= 1 && col_diff <= 1
    }

    ///True if two coordinates are orthogonal (adjacent but not diagonal)
    pub fn is_orthogonal(self, other: Coordinate) -> bool {
        //TODO dont use &
        if self == other {
            return false;
        };

        let row_diff = if self.row > other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };
        let col_diff = if self.column > other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };

        row_diff <= 1 && col_diff <= 1 && (row_diff == 0 || col_diff == 0)
    }

    pub fn get_adjacent_positions<'a, const C: usize, const R: usize>(
        &'a self,
    ) -> impl Iterator<Item = Coordinate> + 'a {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(|(r_offset, c_offset)| {
                let new_row = (self.row as isize) + r_offset;

                if new_row < 0 || new_row as usize >= R {
                    return None;
                }

                let new_col = (self.column as i8) + c_offset;

                if new_col < 0 || new_col as usize >= C {
                    return None;
                }

                let result = Coordinate {
                    row: new_row as usize,
                    column: new_col as usize,
                };

                Some(result)
            })
    }

    pub fn has_at_least_x_neighbors<const C: usize, const R: usize>(&self, x: u8) -> bool {
        if x == 0 {
            return true;
        };
        if x > 8 {
            return false;
        };

        let required_dimensions = match x {
            0 => return true,
            1 => 1,
            2 | 3 => 2,
            4 | 5 => 3,
            6 | 7 | 8 => 4,
            _ => return false,
        };

        let mut dimensions = 0;

        if self.row > 0 {
            dimensions += 1;
        }
        if self.row < R {
            dimensions += 1;
        }

        if self.column > 0 {
            dimensions += 1
        }
        if self.column < C {
            dimensions += 1
        }

        dimensions >= required_dimensions
    }

    pub fn get_positions_up_to<const C: usize, const R: usize>() -> impl Iterator<Item = Coordinate> {
        (0..R)
            .cartesian_product(0..C)
            .map(|(row, column)| Coordinate { row, column })
    }

    pub fn distance_from_centre<const C: usize, const R: usize> (&self) -> usize {
        let d_row = self.row * 2;
        let d_col = self.column * 2;

        let r_dist = if d_row > R {
            d_row - R
        } else {
            R - d_row
        };
        let c_dist = if d_col > C {
            d_col - C
        } else {
            C - d_col
        };

        c_dist + r_dist
    }

    // pub fn get_max_coordinate_for_square_grid(num_nodes: u8) -> Coordinate {
    //     let mut root = num_nodes.sqrt();
    //     if root * root < num_nodes {
    //         root += 1
    //     }

    //     Coordinate {
    //         row: root - 1,
    //         column: root - 1,
    //     }
    // }
}
