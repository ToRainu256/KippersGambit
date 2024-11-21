use chess::{Board, ChessMove, MoveGen};

use std::io::{self, BufRead, Write};
use std::str::FromStr;

const ENGINE_NAME: &str = "KippersGambit";
const ENGINE_AUTHOR: &str = "N.K.James";

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut board = Board::default();
    let mut multipv: usize = 1;

    for line in stdin.lock().lines() {
        let input = line.unwrap();
        #[cfg(debug_assertions)]
        eprintln!("Received command: {}", &input);

        if input == "uci" {
            writeln!(stdout, "id name {}", ENGINE_NAME).unwrap();
            writeln!(stdout, "id author {}", ENGINE_AUTHOR).unwrap();
            writeln!(stdout, "uciok").unwrap();
            stdout.flush().unwrap();
        } else if input == "isready" {
            writeln!(stdout, "readyok").unwrap();
            stdout.flush().unwrap();
        } else if input == "ucinewgame" {
            board = Board::default();
        } else if input.starts_with("position") {
            if input.contains("startpos") {
                board = Board::default();
                if let Some(moves) = input.strip_prefix("position startpos moves ") {
                    for mv in moves.split_whitespace() {
                        if let Ok(chess_move) = mv.parse::<ChessMove>() {
                            board = board.make_move_new(chess_move);
                        }
                    }
                }
            } else if input.contains("fen") {
                if let Some(fen_part) = input.strip_prefix("position fen ") {
                    let parts: Vec<&str> = fen_part.split(" moves ").collect();
                    let fen_str = parts[0];
                    let moves_str = parts.get(1);

                    if let Ok(new_board) = Board::from_str(fen_str) {
                        board = new_board;
                        if let Some(moves) = moves_str {
                            for mv in moves.split_whitespace() {
                                if let Ok(chess_move) = mv.parse::<ChessMove>() {
                                    board = board.make_move_new(chess_move);
                                }
                            }
                        }
                    }
                }
            }
        } else if input.starts_with("setoption") {
            let parts: Vec<&str> = input.splitn(4, ' ').collect();
            if parts.len() >= 4 && parts[1] == "name" {
                let name = parts[2];
                let value_part = input.splitn(2, " value ").collect::<Vec<&str>>();
                let value = if value_part.len() > 1 {
                    value_part[1].trim()
                } else {
                    ""
                };

                if name == "MultiPV" {
                    if let Ok(val) = value.parse::<usize>() {
                        multipv = val.max(1);
                        #[cfg(debug_assertions)]
                        eprintln!("Set MultiPV to {}", multipv);
                    }
                }
            }
        } else if input.starts_with("go") {
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
        } else if input == "quit" {
            break;
        }
    }
}
