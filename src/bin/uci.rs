use std::io;
use std::io::{BufRead, Result, Write};

fn main() -> Result<()> {
    let stdin = io::stdin();
    // * The engine should boot and wait for input from the GUI,
    //   the engine should wait for the "isready" or "setoption" command to set up its internal parameters
    //   as the boot process should be as quick as possible.

    for line in stdin.lock().lines() {
        // * arbitrary white space between tokens is allowed
        //   Example: "debug on\n" and  "   debug     on  \n" and "\t  debug \t  \t\ton\t  \n"
        //   all set the debug mode of the engine on.
        // we might need to trim our line and split whitespace ?
        match line {
            Ok(line) => match line.as_str() {
                "uci" => cmd_uci()?,
                "quit" => break,
                "setoption" => cmd_setoption()?,
                "position" => cmd_position()?,
                "ucinewgame" => cmd_ucinewgame()?,
                "isready" => cmd_isready()?,
                "go" => cmd_go()?,
                "stop" => cmd_stop()?,
                "ponderhit" => cmd_pounderhit()?,
                "debug" => cmd_debug()?,
                "register" => cmd_register()?,
                _ => cmd_unknown(&line)?,
            },
            Err(e) => {
                println!("Error stdin: {}", e);
                break;
            }
        }
        match io::stdout().flush() {
            Ok(()) => {}
            Err(e) => {
                println!("Error flushing stdout: {}", e)
            }
        }
    }
    Ok(())
}

fn cmd_setoption() -> Result<()> {
    // * setoption name <id> [value <x>]
    // this is sent to the engine when the user wants to change the internal parameters
    // of the engine. For the "button" type no value is needed.
    // One string will be sent for each parameter and this will only be sent when the engine is waiting.
    // The name and value of the option in <id> should not be case sensitive and can inlude spaces.
    // The substrings "value" and "name" should be avoided in <id> and <x> to allow unambiguous parsing,
    // for example do not use <name> = "draw value".
    // Here are some strings for the example below:
    //    "setoption name Nullmove value true\n"
    //    "setoption name Selectivity value 3\n"
    //    "setoption name Style value Risky\n"
    //    "setoption name Clear Hash\n"
    //    "setoption name NalimovPath value c:\chess\tb\4;c:\chess\tb\5\n"
    Ok(())
}

fn cmd_position() -> Result<()> {
    // * Before the engine is asked to search on a position, there will always be a position command
    //   to tell the engine about the current position.
    // * position [fen <fenstring> | startpos ]  moves <move1> .... <movei>
    // set up the position described in fenstring on the internal board and
    // play the moves on the internal chess board.
    // if the game was played  from the start position the string "startpos" will be sent
    // Note: no "new" command is needed. However, if this position is from a different game than
    // the last position sent to the engine, the GUI should have sent a "ucinewgame" inbetween.
    Ok(())
}

fn cmd_ucinewgame() -> Result<()> {
    // this is sent to the engine when the next search (started with "position" and "go") will be from
    // a different game. This can be a new game the engine should play or a new game it should analyse but
    // also the next position from a testsuite with positions only.
    // If the GUI hasn't sent a "ucinewgame" before the first "position" command, the engine shouldn't
    // expect any further ucinewgame commands as the GUI is probably not supporting the ucinewgame command.
    // So the engine should not rely on this command even though all new GUIs should support it.
    // As the engine's reaction to "ucinewgame" can take some time the GUI should always send "isready"
    // after "ucinewgame" to wait for the engine to finish its operation.
    Ok(())
}

fn cmd_isready() -> Result<()> {
    let mut stdout = io::stdout();
    // this is used to synchronize the engine with the GUI. When the GUI has sent a command or
    // multiple commands that can take some time to complete,
    // this command can be used to wait for the engine to be ready again or
    // to ping the engine to find out if it is still alive.
    // E.g. this should be sent after setting the path to the tablebases as this can take some time.
    // This command is also required once before the engine is asked to do any search
    // to wait for the engine to finish initializing.
    // This command must always be answered with "readyok" and can be sent also when the engine is calculating
    // in which case the engine should also immediately answer with "readyok" without stopping the search.
    writeln!(stdout, "readyok")?;
    Ok(())
}

fn cmd_go() -> Result<()> {
    // * Before the engine is asked to search on a position, there will always be a position command
    //   to tell the engine about the current position.
    // We must handle error go without position first
    //  * the engine must always be able to process input from stdin, even while thinking.
    // we must add a check in our engine to stop if needed, and add a struct for returns expected by UCI
    // * The engine will always be in forced mode which means it should never start calculating
    //   or pondering without receiving a "go" command first.
    // * if the engine receives a command which is not supposed to come, for example "stop" when the engine is
    //   not calculating, it should also just ignore it.
    //
    //
    //                    start calculating on the current position set up with the "position" command.
    // There are a number of commands that can follow this command, all will be sent in the same string.
    // If one command is not sent its value should be interpreted as it would not influence the search.
    // * searchmoves <move1> .... <movei>
    // 	restrict search to this moves only
    // 	Example: After "position startpos" and "go infinite searchmoves e2e4 d2d4"
    // 	the engine should only search the two moves e2e4 and d2d4 in the initial position.
    // * ponder
    // 	start searching in pondering mode.
    // 	Do not exit the search in ponder mode, even if it's mate!
    // 	This means that the last move sent in in the position string is the ponder move.
    // 	The engine can do what it wants to do, but after a "ponderhit" command
    // 	it should execute the suggested move to ponder on. This means that the ponder move sent by
    // 	the GUI can be interpreted as a recommendation about which move to ponder. However, if the
    // 	engine decides to ponder on a different move, it should not display any mainlines as they are
    // 	likely to be misinterpreted by the GUI because the GUI expects the engine to ponder
    //    on the suggested move.
    // * wtime <x>
    // 	white has x msec left on the clock
    // * btime <x>
    // 	black has x msec left on the clock
    // * winc <x>
    // 	white increment per move in mseconds if x > 0
    // * binc <x>
    // 	black increment per move in mseconds if x > 0
    // * movestogo <x>
    //                         there are x moves to the next time control,
    // 	this will only be sent if x > 0,
    // 	if you don't get this and get the wtime and btime it's sudden death
    // * depth <x>
    // 	search x plies only.
    // * nodes <x>
    //    search x nodes only,
    // * mate <x>
    // 	search for a mate in x moves
    // * movetime <x>
    // 	search exactly x mseconds
    // * infinite
    // 	search until the "stop" command. Do not exit the search without being told so in this mode!
    Ok(())
}

fn cmd_stop() -> Result<()> {
    // * if the engine receives a command which is not supposed to come, for example "stop" when the engine is
    //   not calculating, it should also just ignore it.
    //
    // stop calculating as soon as possible,
    // don't forget the "bestmove" and possibly the "ponder" token when finishing the search
    Ok(())
}

fn cmd_pounderhit() -> Result<()> {
    // the user has played the expected move. This will be sent if the engine was told to ponder on the same move
    // the user has played. The engine should continue searching but switch from pondering to normal search.
    Ok(())
}

fn cmd_unknown(line: &str) -> Result<()> {
    let mut stdout = io::stdout();
    // * if the engine or the GUI receives an unknown command or token it should just ignore it and try to
    //   parse the rest of the string in this line.
    //   Examples: "joho debug on\n" should switch the debug mode on given that joho is not defined,
    //             "debug joho on\n" will be undefined however.
    // pour chaque line un splitword
    writeln!(stdout, "cmd not found: {line}")?;
    Ok(())
}

fn cmd_debug() -> Result<()> {
    // * debug [ on | off ]
    // switch the debug mode of the engine on and off.
    // In debug mode the engine should send additional infos to the GUI, e.g. with the "info string" command,
    // to help debugging, e.g. the commands that the engine has received etc.
    // This mode should be switched off by default and this command can be sent
    // any time, also when the engine is thinking.
    Ok(())
}

fn cmd_register() -> Result<()> {
    // this is the command to try to register an engine or to tell the engine that registration
    // will be done later. This command should always be sent if the engine	has sent "registration error"
    // at program startup.
    // The following tokens are allowed:
    // * later
    //    the user doesn't want to register the engine now.
    // * name <x>
    //    the engine should be registered with the name <x>
    // * code <y>
    //    the engine should be registered with the code <y>
    // Example:
    //    "register later"
    //    "register name Stefan MK code 4359874324"
    Ok(())
}

fn cmd_uci() -> Result<()> {
    let mut stdout = io::stdout();
    // tell engine to use the uci (universal chess interface),
    // this will be sent once as a first command after program boot
    // to tell the engine to switch to uci mode.
    // After receiving the uci command the engine must identify itself with the "id" command
    // and send the "option" commands to tell the GUI which engine settings the engine supports if any.
    // After that the engine should send "uciok" to acknowledge the uci mode.
    // If no uciok is sent within a certain time period, the engine task will be killed by the GUI.
    writeln!(stdout, "id name ChessGame")?;
    writeln!(stdout, "id author Muffin")?;
    writeln!(stdout, "uciok")?;
    Ok(())
}
