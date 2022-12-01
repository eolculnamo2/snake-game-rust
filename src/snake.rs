// the flow of deps sucks here.. lots of circular deps
use std::collections::LinkedList;
use tui::style::{Style, Color};
use tui::widgets::Cell;

use crate::board::{BOARD_AREA, BOARD_WIDTH, SnakeCell, BOARD_HEIGHT, CellType};
use crate::util::{will_hit_wall, convert_id_to_row_col, convert_row_col_to_id};
use crate::movement::{Direction, self};

const INIT_LENGTH: i32 = 4;

pub enum GameEndReason {
    HitWall,
}

pub fn init_snake_list() ->  LinkedList<SnakeCell> {
    let (head_row, head_col) = convert_id_to_row_col(BOARD_AREA / 2 + BOARD_WIDTH / 2 );

    let head_cell = SnakeCell {
        row_id: head_row,
        col_id: head_col,
    };

    let mut snake = LinkedList::from([head_cell]); 
    build_snake(&mut snake, INIT_LENGTH - 1).to_owned()
}

fn build_snake(snake: &mut LinkedList<SnakeCell>, snake_len_remaining: i32) -> &mut LinkedList<SnakeCell> {
    let tail = snake.back().unwrap_or_else(|| panic!("snake cannot be empty"));
    match snake_len_remaining {
        0 => snake,
        _ if will_hit_wall((tail.row_id, tail.col_id), Direction::Right) => snake,
        _ => {
            snake.push_back(SnakeCell {
                row_id: tail.row_id,
                col_id: tail.col_id - 1,
            });
            build_snake(snake, snake_len_remaining -1)
        }
    }

}

pub struct IterationOkResult {
    pub snake: LinkedList<SnakeCell>,
    pub board: Vec<Vec<CellType>>,
}
pub fn make_iteration(snake: LinkedList<SnakeCell>, current_board: Vec<Vec<CellType>>, direction: Direction) -> Result<IterationOkResult, GameEndReason> {
    let mut snake_clone = snake;
    let mut board_clone = current_board;
    snake_clone.pop_back();
    let head = snake_clone.front().unwrap_or_else(|| panic!("Unable to find snake head"));
    if will_hit_wall((head.row_id, head.col_id), direction.clone()) {
        return Err(GameEndReason::HitWall);
    }
    let new_head = SnakeCell {
        row_id: match direction {
            Direction::Left | Direction::Right => head.row_id,
            Direction::Up => head.row_id - BOARD_HEIGHT,
            Direction::Down => head.row_id + BOARD_HEIGHT,
        },
        col_id: match direction {
            Direction::Up | Direction::Down => head.col_id,
            Direction::Right => head.col_id + 1,
            Direction::Left => head.col_id - 1,
        },
    };


    let tail = snake_clone.back().unwrap_or_else(|| panic!("Unable to find snake tail"));
    let old_tail_id = convert_row_col_to_id((tail.row_id, tail.col_id));
    board_clone[tail.row_id as usize][(tail.col_id - 1)  as usize] = CellType::Blank(old_tail_id);
    board_clone[head.row_id as usize][head.col_id as usize] = CellType::Snake(new_head.clone());

    snake_clone.push_front(new_head);
    Ok(IterationOkResult {
        snake: snake_clone.to_owned(),
        board: board_clone,
    })
}
