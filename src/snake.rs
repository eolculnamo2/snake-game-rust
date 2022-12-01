// the flow of deps sucks here.. lots of circular deps
use std::collections::LinkedList;
use crate::board::{BOARD_AREA,SnakeCell};
use crate::util::{will_hit_wall, convert_id_to_row_col};
use crate::movement::Direction;

const INIT_LENGTH: i32 = 4;

fn init_snake_list<'a>() ->  &'a mut LinkedList<SnakeCell> {
    let (head_row, head_col) = convert_id_to_row_col(BOARD_AREA / 2);

    let head_cell = SnakeCell {
        row_id: head_row,
        col_id: head_col,
    };

    let mut snake = LinkedList::from([head_cell]);
    build_snake(&mut snake, INIT_LENGTH)

}

fn build_snake(snake: &mut LinkedList<SnakeCell>, snake_len_remaining: i32) -> &mut LinkedList<SnakeCell> {
    let tail = snake.back().unwrap_or_else(|| panic!("snake cannot be empty"));
    match snake_len_remaining {
        0 => snake,
        _ if will_hit_wall((tail.row_id, tail.col_id), Direction::Right) == false => snake,
        _ => {
            snake.push_back(SnakeCell {
                row_id: tail.row_id,
                col_id: tail.col_id - 1,
            });
            build_snake(snake, snake_len_remaining -1)
        }
    }

}
