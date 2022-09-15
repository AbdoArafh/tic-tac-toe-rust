use colored::Colorize;
use fltk::{
    app::{App, Receiver, Sender},
    prelude::*,
};
use std::fmt;
use std::vec;
pub mod ai;
pub mod ui;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
    X,
    O,
    EMPTY,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type InternalBoard = Vec<Vec<Player>>;

pub struct Board {
    pub board: InternalBoard,
    pub latest_move: Option<Cell>,
}

pub type Cell = (u8, u8);

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![
                vec![Player::EMPTY, Player::EMPTY, Player::EMPTY],
                vec![Player::EMPTY, Player::EMPTY, Player::EMPTY],
                vec![Player::EMPTY, Player::EMPTY, Player::EMPTY],
            ],
            latest_move: None,
        }
    }

    pub fn from(board: InternalBoard) -> Board {
        Board {
            board,
            latest_move: None,
        }
    }

    pub fn print(&self) {
        let mut buf = String::new();
        for row in self.board.iter() {
            for cell in row {
                buf += match cell {
                    Player::X => "X ",
                    Player::O => "O ",
                    Player::EMPTY => "* ",
                }
            }
            buf += "\n";
        }
        println!("{buf}");
    }

    pub fn play_move(&mut self, cell: Cell) -> bool {
        if cell.0 > 2 || cell.1 > 2 || self.board[cell.0 as usize][cell.1 as usize] != Player::EMPTY
        {
            // eprintln!("{}", "Invalid board index provided!".bold().red());
            // process::exit(1);
            return false;
        }

        self.board[cell.0 as usize][cell.1 as usize] = self.player();
        true
    }

    pub fn winner(&self) -> Player {
        for player in [Player::X, Player::O] {
            for row in self.board.iter() {
                if row.iter().all(|cell| *cell == player) {
                    return player;
                }
            }

            for i in 0..3 {
                if vec![&self.board[0][i], &self.board[1][i], &self.board[2][i]]
                    .iter()
                    .all(|cell| **cell == player)
                {
                    return player;
                }
            }

            if vec![&self.board[0][0], &self.board[1][1], &self.board[2][2]]
                .iter()
                .all(|cell| **cell == player)
            {
                return player;
            }

            if vec![&self.board[0][2], &self.board[1][1], &self.board[2][0]]
                .iter()
                .all(|cell| **cell == player)
            {
                return player;
            }
        }

        Player::EMPTY
    }

    pub fn player(&self) -> Player {
        let mut x_count: usize = 0;
        let mut o_count: usize = 0;

        for row in self.board.iter() {
            for cell in row {
                match cell {
                    Player::X => x_count += 1,
                    Player::O => o_count += 1,
                    Player::EMPTY => {}
                }
            }
        }

        if x_count == o_count {
            return Player::X;
        }

        Player::O
    }

    pub fn terminal(&self) -> bool {
        if self.winner() != Player::EMPTY {
            return true;
        }

        let mut empty_count = 0;

        for row in self.board.iter() {
            empty_count += row.iter().filter(|cell| **cell == Player::EMPTY).count();
        }

        empty_count == 0
    }

    pub fn possible_moves(&self, player: Player) -> Vec<Self> {
        let mut empty_cells: Vec<Cell> = Vec::new();

        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Player::EMPTY {
                    empty_cells.push((i as u8, j as u8));
                }
            }
        }

        let mut possible_moves: Vec<Board> = vec![];

        for empty_cell in empty_cells {
            let mut _board = self.board[..].to_vec();
            _board[empty_cell.0 as usize][empty_cell.1 as usize] = player.clone();
            let mut final_board = Self::from(_board);
            final_board.latest_move = Some(empty_cell);
            possible_moves.push(final_board);
        }

        possible_moves
    }

    pub fn reset(&mut self) {
        for row in &mut self.board {
            for cell in row {
                *cell = Player::EMPTY;
            }
        }
    }
}

pub struct Game {
    board: Board,
    buttons: Vec<ui::CellButton>,
    r: Receiver<ui::Message>,
    s: Sender<ui::Message>,
    app: App,
    ai: ai::Ai,
}

impl Game {
    pub fn new() -> Game {
        let game_ui = ui::UI::new("Tic Tac Toe");
        Game {
            board: Board::new(),
            buttons: game_ui.buttons,
            r: game_ui.r,
            s: game_ui.s,
            app: game_ui.app,
            ai: ai::Ai::new(),
        }
    }

    pub fn init(&mut self) {
        for btn in &mut self.buttons {
            btn.button.emit(self.s, ui::Message::Move(btn.coords));
        }
    }

    pub fn listen_to_messages(&mut self) {
        while self.app.wait() {
            if let Some(value) = self.r.recv() {
                match value {
                    ui::Message::Move(cell) => {
                        let player = self.board.player();

                        if self.board.terminal() {
                            for button in &mut self.buttons {
                                button.set_player(Player::EMPTY);
                            }

                            self.board.reset();

                            continue;
                        }

                        let cell = if player == Player::X {
                            self.ai.best_move(&self.board)
                        } else {
                            cell
                        };

                        if !self.play(cell) {
                            continue;
                        }

                        let btn = &mut self.buttons[(cell.0 * 3 + cell.1) as usize];

                        btn.set_player(player);
                    }
                }
            }
        }
    }

    pub fn play(&mut self, cell: Cell) -> bool {
        self.board.play_move(cell)
    }

    pub fn print_status(&self) {
        self.board.print();

        println!(
            "Is terminal state: {}",
            (if self.board.terminal() {
                "Yes".green()
            } else {
                "No".red()
            })
            .bold()
        );
        println!(
            "Current Player: {}",
            self.board.player().to_string().blue().bold()
        );
        println!(
            "Winner: {}",
            match self.board.winner() {
                Player::EMPTY => Player::EMPTY.to_string().dimmed().bold(),
                Player::X => Player::X.to_string().green().bold(),
                Player::O => Player::O.to_string().green().bold(),
            }
        );
    }
}
