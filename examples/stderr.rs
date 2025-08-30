const TEXT: &str = r#"
This screen is ran on stderr.
And when you hit enter, it prints on stdout.
This makes it possible to run an application and choose what will
be sent to any application calling yours.

For example, assuming you build this example with

    cargo build --bin stderr

and then you run it with4

    cd "$(target/debug/stderr)"

what the application prints on stdout is used as argument to cd.

Try it out.

Hit any key to quit this screen:

1 will print `..`
2 will print `/`
3 will print `~`
Any other key will print this text (so that you may copy-paste)
"#;
use std::io;
use std::io::Write;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{execute, queue, terminal};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

fn run_app<W:io::Write>(write: &mut W) -> io::Result<(char)>{
    queue!(
        write,
        EnterAlternateScreen,// 让终端进入备用屏幕
        Hide // 隐藏鼠标
    );
    let mut y=1;
    for line in TEXT.split('\n') {
        queue!(
            write,
            MoveTo(0, y),
            Print(line.to_string()),
        )?;
        y+=1;
    }
    write.flush()?;
    enable_raw_mode()?;
    let user_cher=read_char()?;
    execute!(
        write,
        Show,
        LeaveAlternateScreen
    );
    disable_raw_mode()?;
    Ok((user_cher))
}

fn read_char() -> io::Result<char> {
    loop{
        if let Event::Key(KeyEvent { code:KeyCode::Char(c),.. }) = read()? {
            return Ok(c)
        }
    }
}

fn main() {
    match run_app(&mut io::stderr()).unwrap() {
        '1'=>println!(".."),
        '2'=>println!("/"),
        '3'=>println!("~"),
        _=>println!("{TEXT}")
    };
}

