use std::cmp::max;
use crate::board::{BOARD_WIDTH, BOARD_HEIGHT, BOARD_AREA};
use crate::movement::Direction;

type RowCol = (i32, i32);
pub fn convert_id_to_row_col(id: i32) -> RowCol {
    if id > BOARD_AREA {
        panic!("ID must be less than board area")
    }
    let row_index = max(id / BOARD_HEIGHT - 1, 0);
    let col_index = id % 1;
    (row_index, col_index)
}

pub fn will_hit_wall(location: RowCol, direction: Direction) -> bool {
    let (row_id, col_id) = location;
    match direction {
        Left => col_id % BOARD_WIDTH == 0,
        Right => col_id + 1 % BOARD_WIDTH == 0,
        Up => row_id == 0,
        Down => row_id == BOARD_HEIGHT - 1,
    }
}
