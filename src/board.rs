use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::widgets::{Block, Row, Cell, Table};

pub const BOARD_WIDTH: i32 = 20;
pub const BOARD_HEIGHT: i32 = 20;
pub const BOARD_AREA: i32 = BOARD_WIDTH * BOARD_HEIGHT;

type CellId = i32;

#[derive(Clone)]
pub struct SnakeCell {
    pub row_id: CellId,
    pub col_id: CellId,
}


#[derive(Clone)]
pub enum CellType {
    Blank(CellId),
    Snake(SnakeCell),
}

fn init_board_vector()-> Vec<Vec<CellType>> {
    let col: Vec<CellType> = vec![CellType::Blank(0); BOARD_WIDTH as usize];
    let mut rows = vec![col; BOARD_HEIGHT as usize];

    // todo add snake initial positions here
    for i in 0..rows.len() {
       for j in 0 ..rows[i].len() {
            rows[i][j] = CellType::Blank(BOARD_WIDTH * i as i32 + j as i32);
       }
    }
    rows
}

// UI
fn build_rows<'a>() -> Vec<Row<'a>> {
    let board_vector = init_board_vector();
    let mut rows: Vec<Row<'a>> = vec![];
    for i in 0..board_vector.len() {
        let row = board_vector.get(i).unwrap();

        let tui_cells = row.iter().map(|c| {
            let id = match c {
                CellType::Blank(id) => *id,
                CellType::Snake(_) => 0,
            };
            Cell::from(id.to_string()).style(Style::default().bg(Color::DarkGray))
        })
        .collect::<Vec<Cell<'a>>>();
        let tui_row = Row::new(tui_cells);
        rows.push(tui_row);
    }
    rows
}

pub fn build_board_table<'a>(width_constraints: &'a[Constraint; BOARD_WIDTH as usize]) -> Table<'a> {
    Table::new(build_rows())
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Rust Snake Game"))
        // todo map constraints to board width
        .widths(width_constraints)
        .column_spacing(0)
}
