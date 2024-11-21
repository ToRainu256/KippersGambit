use chess::{Board, ChessMove, MoveGen};

use std::io::{self, BufRead, Write};
use std::str::FromStr;

const ENGINE_NAME: &str = "KippersGambit";
const ENGINE_AUTHOR: &str = "N.K.James";

enum Command {
    Uci,
    IsReady,
    UciNewGame,
    Position {
        fen: Option<String>,
        moves: Vec<String>,
    },
    SetOption {
        name: String,
        value: String,
    },
    Go,
    Quit,
    Unknown(String),
}

fn parse_command(input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();
    match parts.next() {
        Some("uci") => Command::Uci,
        Some("isready") => Command::IsReady,
        Some("ucinewgame") => Command::UciNewGame,
        Some("position") => match parts.next() {
            Some("startpos") => {
                let moves = parts
                    .skip_while(|&s| s != "moves")
                    .skip(1)
                    .map(String::from)
                    .collect();
                Command::Position { fen: None, moves }
            }
            Some("fen") => {
                let fen_parts: Vec<&str> = parts.clone().take_while(|&s| s != "moves").collect();
                let fen_str = fen_parts.join(" ");
                let moves = parts
                    .skip_while(|&s| s != "moves")
                    .skip(1)
                    .map(String::from)
                    .collect();
                Command::Position {
                    fen: Some(fen_str),
                    moves,
                }
            }
            _ => Command::Unknown(input.to_string()),
        },
        Some("setoption") => {
            let name_idx = input.find("name").map(|idx| idx + 4);
            let value_idx = input.find("value").map(|idx| idx + 5);
            if let Some(name_start) = name_idx {
                let name_end = value_idx.unwrap_or_else(|| input.len());
                let name = input[name_start..name_end].trim().to_string();
                let value = value_idx
                    .map(|v_idx| input[v_idx..].trim().to_string())
                    .unwrap_or_default();
                Command::SetOption { name, value }
            } else {
                Command::Unknown(input.to_string())
            }
        }
        Some("go") => Command::Go,
        Some("quit") => Command::Quit,
        _ => Command::Unknown(input.to_string()),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut board = Board::default();
    let mut multipv: usize = 1;

    for line in stdin.lock().lines() {
        let input = line.unwrap();
        #[cfg(debug_assertions)]
        eprintln!("Received command: {}", &input);

        let command = parse_command(&input);

        match command {
            Command::Uci => {
                writeln!(stdout, "id name {}", ENGINE_NAME).unwrap();
                writeln!(stdout, "id author {}", ENGINE_AUTHOR).unwrap();
                writeln!(stdout, "uciok").unwrap();
                stdout.flush().unwrap();
            }
            Command::IsReady => {
                writeln!(stdout, "readyok").unwrap();
                stdout.flush().unwrap();
            }
            Command::UciNewGame => {
                board = Board::default();
            }
            Command::Position { fen, moves } => {
                if let Some(fen_str) = fen {
                    if let Ok(new_board) = Board::from_str(&fen_str) {
                        board = new_board;
                    }
                } else {
                    board = Board::default();
                }
                for mv in moves {
                    if let Ok(chess_move) = mv.parse::<ChessMove>() {
                        board = board.make_move_new(chess_move);
                    }
                }
            }
            Command::SetOption { name, value } => {
                if name == "MultiPV" {
                    if let Ok(val) = value.parse::<usize>() {
                        multipv = val.max(1);
                        #[cfg(debug_assertions)]
                        eprintln!("Set MultiPV to {}", multipv);
                    }
                }
            }
            Command::Go => {
                let legal_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
                if !legal_moves.is_empty() {
                    let pv_moves = legal_moves
                        .iter()
                        .take(multipv)
                        .collect::<Vec<&ChessMove>>();

                    writeln!(stdout, "info depth 1 score cp 0").unwrap();
                    for (i, mv) in pv_moves.iter().enumerate() {
                        writeln!(stdout, "info multipv {} pv {}", i + 1, mv).unwrap();
                    }
                    writeln!(stdout, "bestmove {}", pv_moves[0]).unwrap();
                } else {
                    writeln!(stdout, "bestmove 0000").unwrap();
                }
                stdout.flush().unwrap();
            }
            Command::Quit => {
                break;
            }
            Command::Unknown(cmd) => {
                #[cfg(debug_assertions)]
                eprintln!("Unknown command: {}", cmd);
            }
        }
    }
}
