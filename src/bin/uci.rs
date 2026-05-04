use std::io;
use std::io::{BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => match line.as_str() {
                "uci" => {
                    writeln!(stdout, "id name ChessGame")?;
                    writeln!(stdout, "id author Muffin")?;
                    writeln!(stdout, "uciok")?;
                }
                "quit" => {}
                "setoption" => {}
                "position" => {}
                "ucinewgame" => {}
                "isready" => {
                    writeln!(stdout, "readyok")?;
                }
                "go" => {}
                "stop" => {}
                "ponderhit" => {}
                _ => {
                    writeln!(stdout, "cmd not found: {line}")?;
                }
            },
            Err(e) => {
                println!("Error stdin: {}", e);
                break;
            }
        }
        match stdout.flush() {
            Ok(()) => {}
            Err(e) => {
                println!("Error flushing stdout: {}", e)
            }
        }
    }
    Ok(())
}
