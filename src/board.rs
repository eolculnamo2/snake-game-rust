use std::collections::LinkedList;

use crate::snake::init_snake_list;
use crate::util::{find_cell_in_snake, convert_row_col_to_id, get_random_cell, RowCol, convert_id_to_row_col};
use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::widgets::{Block, Cell, Row, Table};

pub const BOARD_WIDTH: i32 = 20;
pub const BOARD_HEIGHT: i32 = 20;
pub const BOARD_AREA: i32 = BOARD_WIDTH * BOARD_HEIGHT;

type CellId = i32;
const DEBUG: bool = false;

#[derive(Clone)]
pub struct SnakeCell {
    pub row_id: CellId,
    pub col_id: CellId,
}

#[derive(Clone)]
pub enum CellType {
    Blank(CellId),
    Apple(RowCol),
    Snake(SnakeCell),
}

fn get_random_apple_id(board: &Vec<Vec<CellType>>) -> RowCol {
    match get_random_cell(board) {
        CellType::Blank(id) => convert_id_to_row_col(id),
        _ => get_random_apple_id(board),
    }
}
pub fn init_board_vector(snake: LinkedList<SnakeCell>) -> Vec<Vec<CellType>> {
    let col: Vec<CellType> = vec![CellType::Blank(0); BOARD_WIDTH as usize];
    let mut rows = vec![col; BOARD_HEIGHT as usize];

    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            rows[i][j] = match find_cell_in_snake((i as i32, j as i32), snake.clone()) {
                None => CellType::Blank(BOARD_WIDTH * i as i32 + j as i32),
                Some(snake_cell) => CellType::Snake(snake_cell),
            }
        }
    }
    let (apple_row, apple_col) = get_random_apple_id(&rows);
    rows[apple_row as usize][apple_col as usize] = CellType::Apple((apple_row, apple_col));
    rows
}

// UI
fn build_rows<'a>(board_vector: Vec<Vec<CellType>>) -> Vec<Row<'a>> {
    let mut rows: Vec<Row<'a>> = vec![];
    for i in 0..board_vector.len() {
        let row = board_vector.get(i).unwrap();

        let tui_cells = row
            .iter()
            .map(|c| {
                let id = match c {
                    CellType::Blank(id) => *id,
                    CellType::Apple(row_col) => convert_row_col_to_id(*row_col),
                    CellType::Snake(snake_cell) => convert_row_col_to_id((snake_cell.row_id, snake_cell.col_id)),
                };
                let cell_from = if DEBUG {
                    id.to_string()
                } else {
                    String::from("")
                };
                Cell::from(cell_from).style(Style::default().bg(match c {
                    CellType::Blank(_) => Color::DarkGray,
                    CellType::Apple(_) => Color::Magenta,
                    CellType::Snake(_) => Color::Green,
                }))
            })
            .collect::<Vec<Cell<'a>>>();
        let tui_row = Row::new(tui_cells).height(2);
        rows.push(tui_row);
    }
    rows
}

pub fn build_board_table<'a>(
    width_constraints: &'a [Constraint; BOARD_WIDTH as usize],
    board_vector: Vec<Vec<CellType>>
) -> Table<'a> {
    Table::new(build_rows(board_vector))
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Rust Snake Game"))
        // todo map constraints to board width
        .widths(width_constraints)
        .column_spacing(0)
}
