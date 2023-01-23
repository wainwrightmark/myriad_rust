// use itertools::Itertools;
// use serde::{Deserialize, Serialize};
// use std::fmt;

// #[derive(PartialEq, Debug, PartialOrd, Eq, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
// pub struct Coordinate<const C: u8, const R: u8>(pub u8);

// static_assertions::assert_eq_size!(Coordinate<3,3>, u8);

// impl<const C: u8, const R: u8> fmt::Display for Coordinate<C, R> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({},{})", self.get_column(), self.get_row())
//     }
// }

// impl<const C: u8, const R: u8> Coordinate<C, R> {
//     pub fn get_row(&self) -> usize {
//         (self.0 as usize) / C
//     }

//     pub fn get_column(&self) -> usize {
//         (self.0 as usize) % C
//     }

//     pub fn rotate_and_flip(&self, rotation: i8, flip: bool) -> Self {
//         let max = std::cmp::max(C, R) - 1;

//         let rot = rotation.rem_euclid(4);

//         if rot == 0 {
//             return *self;
//         }

//         let (row, column) = match rot {
//             1 => (max - self.get_column(), self.get_row()),
//             2 => (max - self.get_row(), max - self.get_column()),
//             3 => (self.get_column(), max - self.get_row()),
//             _ => panic!("Value should be 0, 1,2,3"),
//         };

//         let rotated = Self::create(column, row);

//         if flip {
//             return rotated.reflect_column();
//         }

//         rotated
//     }

//     pub fn reflect_column(&self) -> Self {
//         let row = self.get_row();
//         let column = C - 1 - self.get_column();

//         Self::create(column, row)
//     }

//     pub fn get_transform(&self, target: Self) -> Option<(i8, bool)> {
//         for flip in [false, true] {
//             for rotation in [0, 1, 2, 3] {
//                 let r = self.rotate_and_flip(rotation, flip);

//                 if r == target {
//                     return Some((rotation, flip));
//                 }
//             }
//         }

//         None
//     }

//     pub fn get_angle(&self, other: Self) -> f64 {
//         let x_diff = other.get_column() as f64 - self.get_column() as f64;
//         let y_diff = other.get_row() as f64 - self.get_row() as f64;

//         (y_diff).atan2(x_diff).to_degrees()
//     }

//     ///True if two coordinates are orthogonal or diagonal
//     pub fn is_adjacent(self, other: Self) -> bool {
//         if self == other {
//             return false;
//         };

//         let row_diff = if self.get_row() > other.get_row() {
//             self.get_row() - other.get_row()
//         } else {
//             other.get_row() - self.get_row()
//         };
//         let col_diff = if self.get_column() > other.get_column() {
//             self.get_column() - other.get_column()
//         } else {
//             other.get_column() - self.get_column()
//         };

//         row_diff <= 1 && col_diff <= 1
//     }

//     ///True if two coordinates are orthogonal (adjacent but not diagonal)
//     pub fn is_orthogonal(self, other: Self) -> bool {
//         if self == other {
//             return false;
//         };

//         let row_diff = if self.get_row() > other.get_row() {
//             self.get_row() - other.get_row()
//         } else {
//             other.get_row() - self.get_row()
//         };
//         let col_diff = if self.get_column() > other.get_column() {
//             self.get_column() - other.get_column()
//         } else {
//             other.get_column() - self.get_column()
//         };

//         row_diff <= 1 && col_diff <= 1 && (row_diff == 0 || col_diff == 0)
//     }

//     pub fn get_adjacent_positions<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
//         (-1..=1)
//             .cartesian_product(-1..=1)
//             .filter_map(|(r_offset, c_offset): (isize, isize)| {
//                 let new_row = (self.get_row() as isize) + r_offset;

//                 if new_row < 0 || new_row as usize >= R {
//                     return None;
//                 }

//                 let new_col = (self.get_column() as isize) + c_offset;

//                 if new_col < 0 || new_col as usize >= C {
//                     return None;
//                 }

//                 let result = Self::create(new_col as usize, new_row as usize);

//                 Some(result)
//             })
//     }

//     pub fn has_at_least_x_neighbors(&self, x: u8) -> bool {
//         if x == 0 {
//             return true;
//         };
//         if x > 8 {
//             return false;
//         };

//         let required_dimensions = match x {
//             0 => return true,
//             1 => 1,
//             2 | 3 => 2,
//             4 | 5 => 3,
//             6 | 7 | 8 => 4,
//             _ => return false,
//         };

//         let mut dimensions = 0;

//         if self.get_row() > 0 {
//             dimensions += 1;
//         }
//         if self.get_row() < R {
//             dimensions += 1;
//         }

//         if self.get_column() > 0 {
//             dimensions += 1
//         }
//         if self.get_column() < C {
//             dimensions += 1
//         }

//         dimensions >= required_dimensions
//     }

//     pub fn get_positions_up_to() -> impl Iterator<Item = Self> {
//         (0..R)
//             .cartesian_product(0..C)
//             .map(|(row, column)| Self::create(column, row))
//     }

//     pub fn distance_from_centre(&self) -> usize {
//         let d_row = self.get_row() * 2;
//         let d_col = self.get_column() * 2;

//         let r_dist = if d_row > R { d_row - R } else { R - d_row };
//         let c_dist = if d_col > C { d_col - C } else { C - d_col };

//         c_dist + r_dist
//     }

//     pub fn create(column: usize, row: usize) -> Self {
//         Self((row * C + column) as u8)
//     }
// }
