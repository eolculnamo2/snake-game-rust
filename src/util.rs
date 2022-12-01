use std::cmp::max;
use std::collections::LinkedList;
use crate::board::{BOARD_WIDTH, BOARD_HEIGHT, BOARD_AREA, SnakeCell};
use crate::movement::Direction;

type RowCol = (i32, i32);
pub fn convert_id_to_row_col(id: i32) -> RowCol {
    if id > BOARD_AREA {
        panic!("ID must be less than board area")
    }
    let row_index = max(id / BOARD_HEIGHT, 0);
    let col_index = id % BOARD_WIDTH;
    (row_index, col_index)
}

pub fn will_hit_wall(location: RowCol, direction: Direction) -> bool {
    let (row_id, col_id) = location;
    match direction {
        Direction::Left => col_id % BOARD_WIDTH == 0,
        Direction::Right => col_id + 1 % BOARD_WIDTH == 0,
        Direction::Up => row_id == 0,
        Direction::Down => row_id == BOARD_HEIGHT - 1,
    }
}

pub fn find_cell_in_snake(location: RowCol, snake: LinkedList<SnakeCell>) -> Option<SnakeCell> {
    let (row_id, col_id) = location;
    let mut snake_iter = snake.iter();
    let mut next = snake_iter.next();
    let mut matched_cell: Option<SnakeCell> = None;
        
    while let Some(cell) = next {
        if row_id == cell.row_id && col_id == cell.col_id {
            matched_cell = Some(cell.clone());
            break
        }
        next = snake_iter.next();
    }

    matched_cell
}
