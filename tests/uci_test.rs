use chess::{Board, ChessMove};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::{io, thread};

fn start_engine(engine_path: &str) -> io::Result<Child> {
    Command::new(engine_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
}

fn send_command(stdin: &mut impl Write, command: &str) -> io::Result<()> {
    writeln!(stdin, "{}", command)?;
    stdin.flush()
}

fn read_lines(reader: &mut impl BufRead, timeout: Duration) -> io::Result<Vec<String>> {
    let start = std::time::Instant::now();
    let mut lines = Vec::new();
    while start.elapsed() < timeout {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            // EOF
            break;
        }
        let line = buffer.trim().to_string();
        println!("Engine: {}", line);
        lines.push(line.clone());
    }
    Ok(lines)
}

fn wait_for_keyword(
    reader: &mut impl BufRead,
    keyword: &str,
    timeout: Duration,
) -> io::Result<Vec<String>> {
    let start = std::time::Instant::now();
    let mut lines = Vec::new();
    while start.elapsed() < timeout {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            // EOF
            break;
        }
        let line = buffer.trim().to_string();
        println!("Engine: {}", line);
        lines.push(line.clone());
        if line.contains(keyword) {
            return Ok(lines);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        format!("Timeout waiting for '{}'", keyword),
    ))
}

#[test]
fn test_uci_engine() -> io::Result<()> {
    let engine_path = "./target/release/KippersGambit";
    let mut child = start_engine(engine_path)?;

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    // UCI
    send_command(stdin, "uci")?;
    let lines = wait_for_keyword(&mut reader, "uciok", Duration::from_secs(2))?;
    assert!(
        lines.iter().any(|line| line == "uciok"),
        "Engine did not respond with 'uciok'"
    );

    send_command(stdin, "isready")?;
    let lines = wait_for_keyword(&mut reader, "readyok", Duration::from_secs(2))?;
    assert!(
        lines.iter().any(|line| line == "readyok"),
        "Engine did not respond with 'readyok'"
    );

    send_command(stdin, "ucinewgame")?;
    send_command(stdin, "isready")?;
    let lines = wait_for_keyword(&mut reader, "readyok", Duration::from_secs(2))?;
    assert!(
        lines.iter().any(|line| line == "readyok"),
        "Engine did not respond with 'readyok' after ucinewgame"
    );

    send_command(stdin, "position startpos moves e2e4 e7e5")?;
    send_command(stdin, "go")?;

    let lines = wait_for_keyword(&mut reader, "bestmove", Duration::from_secs(5))?;
    let bestmove_line = lines
        .iter()
        .find(|line| line.starts_with("bestmove"))
        .expect("No bestmove found");
    println!("Best Move: {}", bestmove_line);

    let parts: Vec<&str> = bestmove_line.split_whitespace().collect();
    assert!(
        parts.len() >= 2,
        "Invalid bestmove format: {}",
        bestmove_line
    );
    let best_move_str = parts[1];
    assert!(
        best_move_str.len() == 4 || best_move_str.len() == 5,
        "Invalid move length: {}",
        best_move_str
    );
    let board =
        Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string())
            .unwrap();
    let mv = best_move_str.parse::<ChessMove>();
    assert!(mv.is_ok(), "Invalid move notation: {}", best_move_str);
    assert!(
        board.legal(*mv.as_ref().unwrap()),
        "Engine suggested an illegal move: {}",
        best_move_str
    );

    // quitコマンドを送信
    send_command(stdin, "quit")?;

    child.wait()?;
    Ok(())
}
