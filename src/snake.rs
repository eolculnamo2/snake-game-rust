// the flow of deps sucks here.. lots of circular deps
use std::collections::LinkedList;
use crate::board::{BOARD_AREA, BOARD_WIDTH, SnakeCell, CellType, get_random_apple_id};
use crate::util::{will_hit_wall, convert_id_to_row_col, convert_row_col_to_id};
use crate::movement::{Direction};

const INIT_LENGTH: i32 = 4;

pub enum GameEndReason {
    HitSelf,
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

#[derive(PartialEq, Eq)]
enum CollisionResult  {
    NoOp,
    GrowSnake,
    GameEnd,
}

pub struct IterationOkResult {
    pub snake: LinkedList<SnakeCell>,
    pub board: Vec<Vec<CellType>>,
    pub game_end: Option<GameEndReason>,
}
pub fn make_iteration(snake: LinkedList<SnakeCell>, current_board: Vec<Vec<CellType>>, direction: Direction) -> Result<IterationOkResult, GameEndReason> {
    let mut snake_clone = snake;
    let mut board_clone = current_board;
    let head = snake_clone.front().cloned().unwrap_or_else(|| panic!("Unable to find snake head"));
    if will_hit_wall((head.row_id, head.col_id), direction.clone()) {
        return Err(GameEndReason::HitWall);
    }
    let new_head = SnakeCell {
        row_id: match direction {
            Direction::Up => head.row_id - 1,
            Direction::Down => head.row_id + 1,
            _ => head.row_id,
        },
        col_id: match direction {
            Direction::Right => head.col_id + 1,
            Direction::Left => head.col_id - 1,
            _ => head.col_id,
        },
    };

    // handle head to new head collision
    let collision_result = match board_clone[new_head.row_id as usize][new_head.col_id as usize] {
        CellType::Blank(_) => CollisionResult::NoOp, // no op
        CellType::Apple(_) => CollisionResult::GrowSnake,
        CellType::Snake(_) => CollisionResult::GameEnd,
    };


    if collision_result != CollisionResult::GrowSnake {
        let old_tail = snake_clone.pop_back().unwrap_or_else(|| panic!("no tail found"));
        let old_tail_id = convert_row_col_to_id((old_tail.row_id, old_tail.col_id));
        board_clone[old_tail.row_id as usize][old_tail.col_id as usize] = CellType::Blank(old_tail_id);
    } 
    board_clone[head.row_id as usize][head.col_id as usize] = CellType::Snake(new_head.clone());

    snake_clone.push_front(new_head);


    if collision_result == CollisionResult::GrowSnake {
        let (apple_row, apple_col) = get_random_apple_id(&board_clone);
        board_clone[apple_row as usize][apple_col as usize] = CellType::Apple((apple_row, apple_col));
    }
    Ok(IterationOkResult {
        snake: snake_clone.to_owned(),
        board: board_clone,
        game_end: if collision_result == CollisionResult::GameEnd {
            Some(GameEndReason::HitSelf)
        } else {
            None
        }
    })
}
