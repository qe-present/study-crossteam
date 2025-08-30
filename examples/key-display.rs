use std::io::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    event::{read}
};
use crossterm::event::{KeyCode, KeyModifiers};

const HELP: &str = r#"Key display
- Press any key to end the execution.
- Use ESC to quit.
"#;
fn main()-> Result<()> {
    println!("{}", HELP);
    enable_raw_mode()?;
    if let Err(e)=print_event(){
        println!("{}", e);
    }
    println!("end the execution.");
    disable_raw_mode()?;
    Ok(())
}
fn print_event()-> Result<()> {
    while let Ok(event)=read(){
        let Some(event)=event.as_key_press_event() else {
            continue;
        };
        let modifiers=match event.modifiers{
            KeyModifiers::NONE=>"".to_string(),
            _=>format!("{:?}", event.modifiers)
        };
        println!("Key press: {modifiers}{code}\r", code=event.code);
        if (event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL) {
            break;
        }
    };
    Ok(())
}